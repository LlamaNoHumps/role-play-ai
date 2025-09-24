use crate::{database::Database, error::HttpResult};
use axum::{Extension, Json, extract::Query};
use serde::Deserialize;
use std::sync::Arc;

pub const PATH: &str = "/api/conversation/list";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Query(RequestParams { user_id }): Query<RequestParams>,
) -> HttpResult<Json<Vec<i32>>> {
    let conversations = database.list_conversations(user_id).await?;
    let conversations = conversations
        .into_iter()
        .map(|conv| conv.role_id)
        .collect::<Vec<i32>>();

    Ok(Json(conversations))
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub user_id: i32,
}
