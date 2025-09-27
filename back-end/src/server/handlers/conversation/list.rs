use crate::{database::Database, error::HttpResult};
use axum::{Extension, Json, extract::Query};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/conversation/list";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Query(params): Query<RequestParams>,
) -> HttpResult<Json<PaginatedResponse>> {
    let conversations = database
        .list_conversations_paginated(
            params.user_id,
            params.offset.unwrap_or(0),
            params.limit.unwrap_or(15),
        )
        .await?;

    let role_ids = conversations
        .items
        .into_iter()
        .map(|conv| conv.role_id)
        .collect::<Vec<i32>>();

    Ok(Json(PaginatedResponse {
        items: role_ids,
        total: conversations.total,
        has_more: conversations.has_more,
    }))
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub user_id: i32,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct PaginatedResponse {
    pub items: Vec<i32>,
    pub total: i64,
    pub has_more: bool,
}
