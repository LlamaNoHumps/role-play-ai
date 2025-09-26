use crate::{database::Database, error::HttpResult};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/user/debates/delete";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Json(RequestParams { user_id }): Json<RequestParams>,
) -> HttpResult<Json<ResponseData>> {
    let deleted_count = database.delete_all_user_debates(user_id).await?;

    Ok(Json(ResponseData {
        success: true,
        message: format!("Deleted {} debates successfully", deleted_count),
        deleted_count,
    }))
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub user_id: i32,
}

#[derive(Serialize)]
pub struct ResponseData {
    pub success: bool,
    pub message: String,
    pub deleted_count: i32,
}
