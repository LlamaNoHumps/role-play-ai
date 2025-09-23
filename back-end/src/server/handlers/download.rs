use crate::{database::Database, error::HttpResult, storage::StorageClient};
use axum::{Extension, extract::Query};
use serde::Deserialize;
use std::sync::Arc;

pub const PATH: &str = "/download";

#[axum::debug_handler]
pub async fn handler(
    Extension(storage_client): Extension<Arc<StorageClient>>,
    Extension(database): Extension<Arc<Database>>,
    Query(RequestParams { voice_id }): Query<RequestParams>,
) -> HttpResult<Vec<u8>> {
    let filename = database.get_voice_filename(voice_id).await?;

    let audio_data = storage_client.download_object(&filename).await?;

    Ok(audio_data)
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub voice_id: i32,
}
