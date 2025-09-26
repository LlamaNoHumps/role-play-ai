use crate::{database::Database, error::HttpResult};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/debate/list";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Json(RequestParams {
        user_id,
        offset,
        limit,
    }): Json<RequestParams>,
) -> HttpResult<Json<ResponseData>> {
    let paginated_result = database
        .list_debates_paginated(user_id, offset, limit)
        .await?;

    let debates: Vec<DebateItem> = paginated_result
        .items
        .into_iter()
        .map(|debate| DebateItem {
            id: debate.id,
            role1_id: debate.role1_id,
            role2_id: debate.role2_id,
            topic: debate.topic,
            last_dialog_timestamp: debate.last_dialog_timestamp,
            current_speaker_id: debate.current_speaker_id,
        })
        .collect();

    Ok(Json(ResponseData {
        debates,
        total: paginated_result.total,
        has_more: paginated_result.has_more,
    }))
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub user_id: i32,
    pub offset: i64,
    pub limit: i64,
}

#[derive(Serialize)]
pub struct ResponseData {
    pub debates: Vec<DebateItem>,
    pub total: i64,
    pub has_more: bool,
}

#[derive(Serialize)]
pub struct DebateItem {
    pub id: i32,
    pub role1_id: i32,
    pub role2_id: i32,
    pub topic: String,
    pub last_dialog_timestamp: i64,
    pub current_speaker_id: i32,
}
