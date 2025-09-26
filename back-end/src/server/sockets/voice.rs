use crate::{
    agents::{
        AI, Recorder,
        reciter::{Reciter, remove_brackets},
    },
    database::Database,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, Extension, SocketRef};
use std::sync::Arc;

pub const EVENT: &str = "voice";

pub async fn handler(
    socket: SocketRef,
    ai: Extension<Arc<AI>>,
    database: Extension<Arc<Database>>,
    recorder: Extension<Recorder>,
    reciter: Extension<Reciter>,
    data: Data<MessageData>,
) {
    if let Err(e) = handler_inner(socket, ai, database, recorder, reciter, data).await {
        tracing::error!("socket voice handler error: {}", e);
    }
}

pub async fn handler_inner(
    socket: SocketRef,
    Extension(ai): Extension<Arc<AI>>,
    Extension(database): Extension<Arc<Database>>,
    Extension(recorder): Extension<Recorder>,
    Extension(reciter): Extension<Reciter>,
    Data(MessageData {
        id,
        user_id,
        role_id,
        timestamp,
        voice_url,
    }): Data<MessageData>,
) -> Result<()> {
    let text = recorder.asr(&voice_url).await?;
    database
        .add_dialog(user_id, role_id, true, timestamp, &text, Some(voice_url))
        .await?;
    socket.emit(
        "update_message",
        &UpdateData {
            id,
            text: text.clone(),
        },
    )?;

    let role = database.get_role(role_id).await?;
    let history = database.get_history(user_id, role_id).await?;

    let answer = ai.chat_once(&role.prompt(), &text, Some(&history)).await?;

    let cleaned_answer = remove_brackets(&answer);
    let audio_data = reciter.tts(&cleaned_answer, &role.voice_type).await?;
    let voice_url = reciter.upload_audio(audio_data).await?;

    let timestamp = chrono::Utc::now().timestamp_millis();
    database
        .add_dialog(
            user_id,
            role_id,
            false,
            timestamp,
            &answer,
            Some(voice_url.clone()),
        )
        .await?;

    socket.emit(
        "message",
        &EmitData {
            role_id,
            timestamp,
            text: answer,
            voice_url,
        },
    )?;

    Ok(())
}

#[derive(Deserialize)]
pub struct MessageData {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub timestamp: i64,
    pub voice_url: String,
}

#[derive(Serialize)]
pub struct UpdateData {
    pub id: i32,
    pub text: String,
}

#[derive(Serialize)]
pub struct EmitData {
    pub role_id: i32,
    pub timestamp: i64,
    pub text: String,
    pub voice_url: String,
}
