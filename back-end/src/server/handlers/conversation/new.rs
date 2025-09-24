use crate::{database::Database, error::HttpResult};
use axum::{Extension, Json};
use serde::Deserialize;
use std::sync::Arc;

pub const PATH: &str = "/api/conversation/new";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Json(RequestParams { user_id, role_id }): Json<RequestParams>,
) -> HttpResult<()> {
    database.create_conversation_table(user_id, role_id).await?;

    Ok(())
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub user_id: i32,
    pub role_id: i32,
}
