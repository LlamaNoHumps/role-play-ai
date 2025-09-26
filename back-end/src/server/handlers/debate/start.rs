use crate::{
    agents::{Debater, Reciter},
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
    Json(RequestParams {
        user_id,
        role1_id,
        role2_id,
    }): Json<RequestParams>,
) -> HttpResult<Json<ResponseData>> {
    let debate = database
        .get_debate(user_id, role1_id, role2_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Debate not found"))?;

    let current_speaker_id = debate.current_speaker_id;

    let role1 = database.get_role(role1_id).await?;
    let role2 = database.get_role(role2_id).await?;

    let (current_role, other_role) = if current_speaker_id == role1_id {
        (role1, role2)
    } else {
        (role2, role1)
    };

    let is_starting = database
        .get_recent_debate_dialogs(user_id, role1_id, role2_id, 1)
        .await?
        .is_empty();

    let response = debater
        .answer(user_id, role1_id, role2_id, current_speaker_id, is_starting)
        .await?;

    let voice_data = reciter.tts(&response, &current_role.voice_type).await?;
    let voice_url = reciter.upload_audio(voice_data).await?;

    let timestamp = chrono::Utc::now().timestamp();
    database
        .add_debate_dialog(
            user_id,
            role1_id,
            role2_id,
            current_speaker_id,
            timestamp,
            &response,
            Some(voice_url.clone()),
        )
        .await?;

    database
        .update_debate_current_speaker_id(user_id, role1_id, role2_id, other_role.id)
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
    pub user_id: i32,
    pub role1_id: i32,
    pub role2_id: i32,
}

#[derive(Serialize)]
pub struct ResponseData {
    pub current_speaker_id: i32,
    pub response: String,
    pub timestamp: i64,
    pub voice_url: String,
}
