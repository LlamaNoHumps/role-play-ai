use crate::{
    agents::{Debater, Reciter, reciter::remove_brackets},
    database::Database,
    error::HttpResult,
};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/debate/start";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Extension(debater): Extension<Debater>,
    Extension(reciter): Extension<Reciter>,
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

    let current_speaker_id = debate.current_speaker_id;

    let role1 = database.get_role(debate.role1_id).await?;
    let role2 = database.get_role(debate.role2_id).await?;

    let (current_role, other_role) = if current_speaker_id == debate.role1_id {
        (role1, role2)
    } else {
        (role2, role1)
    };

    let is_starting = database
        .get_debate_dialogs_by_id(debate.id, 1)
        .await?
        .is_empty();

    let response = debater
        .answer(
            debate.user_id,
            debate.role1_id,
            debate.role2_id,
            current_speaker_id,
            is_starting,
        )
        .await?;

    let cleaned_response = remove_brackets(&response);
    let voice_data = reciter
        .tts(&cleaned_response, &current_role.voice_type)
        .await?;
    let voice_url = reciter.upload_audio(voice_data).await?;

    let timestamp = chrono::Utc::now().timestamp();
    database
        .add_debate_dialog_by_id(
            debate.id,
            current_speaker_id,
            timestamp,
            &response,
            Some(voice_url.clone()),
        )
        .await?;

    database
        .update_debate_current_speaker_id_by_id(debate.id, other_role.id)
        .await?;

    Ok(Json(ResponseData {
        current_speaker_id,
        response,
        timestamp,
        voice_url,
    }))
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub debate_id: Option<i32>, // 新增：通过debate_id查询
    pub user_id: i32,           // 兼容性：通过用户和角色查询
    pub role1_id: Option<i32>,  // 可选：当使用debate_id时不需要
    pub role2_id: Option<i32>,  // 可选：当使用debate_id时不需要
}

#[derive(Serialize)]
pub struct ResponseData {
    pub current_speaker_id: i32,
    pub response: String,
    pub timestamp: i64,
    pub voice_url: String,
}
