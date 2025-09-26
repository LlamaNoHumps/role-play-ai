use crate::{database::Database, error::HttpResult};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/debate/dialogs";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Json(params): Json<RequestParams>,
) -> HttpResult<Json<ResponseData>> {
    let debate = if let Some(debate_id) = params.debate_id {
        database
            .get_debate_by_id(debate_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Debate not found"))?
    } else if let (Some(role1_id), Some(role2_id)) = (params.role1_id, params.role2_id) {
        database
            .get_debate(params.user_id, role1_id, role2_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Debate not found"))?
    } else {
        return Err(
            anyhow::anyhow!("Either debate_id or role1_id/role2_id must be provided").into(),
        );
    };

    let paginated_result = if let Some(debate_id) = params.debate_id {
        database
            .list_debate_dialogs_paginated_by_id(debate_id, params.offset, params.limit)
            .await?
    } else if let (Some(role1_id), Some(role2_id)) = (params.role1_id, params.role2_id) {
        database
            .list_debate_dialogs_paginated(
                params.user_id,
                role1_id,
                role2_id,
                params.offset,
                params.limit,
            )
            .await?
    } else {
        return Err(
            anyhow::anyhow!("Either debate_id or role1_id/role2_id must be provided").into(),
        );
    };

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
    pub debate_id: Option<i32>, // 新增：通过debate_id查询
    pub user_id: i32,           // 兼容性：通过用户和角色查询
    pub role1_id: Option<i32>,  // 可选：当使用debate_id时不需要
    pub role2_id: Option<i32>,  // 可选：当使用debate_id时不需要
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
