use crate::{database::Database, error::HttpResult};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/debate/dialogs";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Json(RequestParams {
        user_id,
        role1_id,
        role2_id,
        offset,
        limit,
    }): Json<RequestParams>,
) -> HttpResult<Json<ResponseData>> {
    let debate = database
        .get_debate(user_id, role1_id, role2_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Debate not found"))?;

    let paginated_result = database
        .list_debate_dialogs_paginated(user_id, role1_id, role2_id, offset, limit)
        .await?;

    let dialogs: Vec<DialogItem> = paginated_result
        .items
        .into_iter()
        .map(|d| DialogItem {
            id: d.id,
            role_id: d.role_id,
            timestamp: d.timestamp,
            text: d.text,
            voice: d.voice,
        })
        .collect();

    Ok(Json(ResponseData {
        debate_id: debate.id,
        topic: debate.topic,
        role1_id: debate.role1_id,
        role2_id: debate.role2_id,
        current_speaker_id: debate.current_speaker_id,
        dialogs,
        total: paginated_result.total,
        has_more: paginated_result.has_more,
    }))
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub user_id: i32,
    pub role1_id: i32,
    pub role2_id: i32,
    pub offset: i64,
    pub limit: i64,
}

#[derive(Serialize)]
pub struct ResponseData {
    pub debate_id: i32,
    pub topic: String,
    pub role1_id: i32,
    pub role2_id: i32,
    pub current_speaker_id: i32,
    pub dialogs: Vec<DialogItem>,
    pub total: i64,
    pub has_more: bool,
}

#[derive(Serialize)]
pub struct DialogItem {
    pub id: i32,
    pub role_id: i32,
    pub timestamp: i64,
    pub text: String,
    pub voice: Option<String>,
}
