use crate::{database::Database, error::HttpResult};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/debate/delete";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Json(RequestParams { user_id, debate_id }): Json<RequestParams>,
) -> HttpResult<Json<ResponseData>> {
    let debate = database
        .get_debate_by_id(debate_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Debate not found"))?;

    if debate.user_id != user_id {
        return Err(anyhow::anyhow!("Permission denied").into());
    }

    database.delete_debate_with_dialogs(debate_id).await?;

    Ok(Json(ResponseData {
        success: true,
        message: "Debate deleted successfully".to_string(),
    }))
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub user_id: i32,
    pub debate_id: i32,
}

#[derive(Serialize)]
pub struct ResponseData {
    pub success: bool,
    pub message: String,
}
