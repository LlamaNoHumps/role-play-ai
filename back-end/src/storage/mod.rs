use anyhow::{Result, anyhow};
use axum::Extension;
use futures::io::Cursor;
use qiniu_sdk::{
    apis::{Client, storage::create_bucket::PathParams},
    credential::Credential,
    download::{DownloadManager, StaticDomainsUrlsGenerator},
    http_client::{AllRegionsProvider, RegionsProviderEndpoints},
    prelude::RegionsProvider,
    upload::{AutoUploader, AutoUploaderObjectParams, UploadManager, UploadTokenSigner},
};
use serde::Deserialize;
use std::{sync::Arc, time::Duration};

pub struct StorageClient {
    credential: Credential,
    client: Client,
    bucket_name: &'static str,
    domain: String,
}

impl StorageClient {
    pub fn new(access_key: &str, secret_key: &str) -> Self {
        const BUCKET_NAME: &str = "role-play-ai";

        let credential = Credential::new(access_key, secret_key);
        let client = Client::default();

        Self {
            credential,
            client,
            bucket_name: BUCKET_NAME,
            domain: String::new(),
        }
    }

    pub async fn init_bucket(&mut self) -> Result<()> {
        if let Some(domain) = self.get_domain().await? {
            self.domain = domain;

            return Ok(());
        }

        let region = AllRegionsProvider::new(self.credential.to_owned())
            .async_get(Default::default())
            .await?;

        self.client
            .storage()
            .create_bucket()
            .new_async_request(
                RegionsProviderEndpoints::new(&region),
                PathParams::default().set_bucket_as_str(self.bucket_name),
                self.credential.to_owned(),
            )
            .call()
            .await?;

        self.domain = self
            .get_domain()
            .await?
            .ok_or(anyhow!("Failed to get domain"))?;

        Ok(())
    }

    pub async fn get_domain(&self) -> Result<Option<String>> {
        let region = AllRegionsProvider::new(self.credential.to_owned())
            .async_get(Default::default())
            .await?;

        let res = self
            .client
            .storage()
            .get_domains()
            .new_async_request(
                RegionsProviderEndpoints::new(&region),
                self.credential.to_owned(),
            )
            .query_pairs([("tbl".into(), self.bucket_name.into())])
            .call()
            .await?;

        let body = res.into_body();
        let body = body.to_str_vec();

        if body.is_empty() {
            return Ok(None);
        }

        Ok(Some(body[0].to_string()))
    }

    pub async fn upload_object(&self, name: &str, data: Vec<u8>) -> Result<ObjectInfo> {
        let upload_manager = UploadManager::builder(UploadTokenSigner::new_credential_provider(
            self.credential.to_owned(),
            self.bucket_name,
            Duration::from_secs(3600),
        ))
        .build();

        let uploader: AutoUploader = upload_manager.auto_uploader();

        let params = AutoUploaderObjectParams::builder()
            .object_name(name)
            .file_name(name)
            .build();

        let res = uploader
            .async_upload_reader(Cursor::new(data), params)
            .await?;

        let object_info = serde_json::from_value::<ObjectInfo>(res)?;

        Ok(object_info)
    }

    pub async fn download_object(&self, name: &str) -> Result<Vec<u8>> {
        let download_manager = DownloadManager::new(
            StaticDomainsUrlsGenerator::builder(self.domain.clone())
                .use_https(false) // 设置为 HTTP 协议
                .build(),
        );

        let mut writer = Vec::new();
        download_manager
            .download(name)?
            .to_async_writer(&mut writer)
            .await?;

        Ok(writer)
    }

    pub fn into_layer(self) -> Extension<Arc<Self>> {
        Extension(Arc::new(self))
    }
}

#[derive(Deserialize)]
pub struct ObjectInfo {
    #[serde(rename = "hash")]
    pub _hash: String,
    pub key: String,
}
