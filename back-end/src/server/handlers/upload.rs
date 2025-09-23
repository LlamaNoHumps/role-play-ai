use crate::{
    database::Database,
    error::HttpResult,
    storage::{ObjectInfo, StorageClient},
};
use axum::{Extension, Json, body::Bytes, http::HeaderMap};
use serde::Serialize;
use std::sync::Arc;

pub const PATH: &str = "/upload";

#[axum::debug_handler]
pub async fn handler(
    Extension(storage_client): Extension<Arc<StorageClient>>,
    Extension(database): Extension<Arc<Database>>,
    headers: HeaderMap,
    body: Bytes,
) -> HttpResult<Json<ResponseData>> {
    let name = headers.get("X-File-Name").unwrap().to_str()?;
    let ObjectInfo { key: filename, .. } =
        storage_client.upload_object(name, body.to_vec()).await?;
    let voice_id = database.add_voice(&filename).await?;

    Ok(Json(ResponseData { voice_id }))
}

#[derive(Serialize)]
pub struct ResponseData {
    pub voice_id: i32,
}
