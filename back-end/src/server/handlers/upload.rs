use crate::{
    error::HttpResult,
    storage::{ObjectInfo, StorageClient},
};
use axum::{Extension, Json, body::Bytes, http::HeaderMap};
use serde::Serialize;
use std::sync::Arc;

pub const PATH: &str = "/api/upload";

#[axum::debug_handler]
pub async fn handler(
    Extension(storage_client): Extension<Arc<StorageClient>>,
    headers: HeaderMap,
    body: Bytes,
) -> HttpResult<Json<ResponseData>> {
    let name = headers.get("X-File-Name").unwrap().to_str()?;

    let uuid = uuid::Uuid::new_v4();
    let name = format!("{}-{}.jpg", name, uuid);
    tokio::fs::write(&name, &body).await?;

    let ObjectInfo { key: filename, .. } =
        storage_client.upload_object_from_file(&name, &name).await?;

    tokio::fs::remove_file(&name).await?;

    let file_url = storage_client.get_object_url(&filename);

    Ok(Json(ResponseData { file_url }))
}

#[derive(Serialize)]
pub struct ResponseData {
    pub file_url: String,
}
