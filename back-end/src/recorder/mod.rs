use crate::storage::StorageClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct Recorder {
    storage_client: Arc<StorageClient>,
    http_client: reqwest::Client,
}

impl Recorder {
    pub fn new(storage_client: Arc<StorageClient>, ai_api_key: &str) -> Self {
        let headers = {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::CONTENT_TYPE,
                "application/json".parse().unwrap(),
            );
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", ai_api_key).parse().unwrap(),
            );
            headers
        };

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            storage_client,
            http_client,
        }
    }

    pub async fn asr(&self, voice_url: &str) -> Result<String> {
        const URL: &str = "https://openai.qiniu.com/v1/voice/asr";

        let data = RequestParams {
            model: "asr".to_string(),
            audio: AudioInfo {
                format: "mp3".to_string(),
                url: voice_url.to_string(),
            },
        };

        let res = self.http_client.post(URL).json(&data).send().await?;

        let body = res.text().await?;
        let res = serde_json::from_str::<Response>(&body)?;

        Ok(res.data.result.text)
    }
}

#[derive(Serialize)]
struct RequestParams {
    model: String,
    audio: AudioInfo,
}

#[derive(Serialize)]
struct AudioInfo {
    format: String,
    url: String,
}

#[derive(Deserialize)]
struct Response {
    reqid: String,
    operation: String,
    data: ResponseData,
}

#[derive(Deserialize)]
struct ResponseData {
    audio_info: DurationI32,
    result: ResponseDataResult,
}

#[derive(Deserialize)]
struct DurationI32 {
    duration: i32,
}

#[derive(Deserialize)]
struct DurationString {
    duration: String,
}

#[derive(Deserialize)]
struct ResponseDataResult {
    additions: DurationString,
    text: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::tests::get_env;

    #[tokio::test]
    async fn test_asr() {
        let env = get_env();

        let mut storage_client = StorageClient::new(&env.qiniu_access_key, &env.qiniu_secret_key);
        storage_client.init_bucket().await.unwrap();
        let recorder = Recorder::new(Arc::new(storage_client), &env.qiniu_ai_api_key);
        let audio_data = recorder.asr("http://t2zfj0z3c.hd-bkt.clouddn.com/audio_e01b5f19-0bd9-4510-a580-fd6c44d14d53.mp3").await.unwrap();

        println!("{}", audio_data);
    }
}
