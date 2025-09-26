use crate::{
    agents::{AI, Summarizer},
    database::Database,
    reciter::{Reciter, remove_brackets},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, Extension, SocketRef};
use std::sync::Arc;

pub const EVENT: &str = "message";

pub async fn handler(
    socket: SocketRef,
    ai: Extension<Arc<AI>>,
    database: Extension<Arc<Database>>,
    reciter: Extension<Reciter>,
    summarizer: Extension<Arc<Summarizer>>,
    data: Data<MessageData>,
) {
    if let Err(e) = handler_inner(socket, ai, database, reciter, summarizer, data).await {
        tracing::error!("socket message handler error: {}", e);
    }
}

pub async fn handler_inner(
    socket: SocketRef,
    Extension(ai): Extension<Arc<AI>>,
    Extension(database): Extension<Arc<Database>>,
    Extension(reciter): Extension<Reciter>,
    Extension(summarizer): Extension<Arc<Summarizer>>,
    Data(MessageData {
        user_id,
        role_id,
        timestamp,
        text,
    }): Data<MessageData>,
) -> Result<()> {
    database
        .add_dialog(user_id, role_id, true, timestamp, &text, None)
        .await?;

    socket.emit(
        "user_message_saved",
        &UserMessageSavedData {
            user_id,
            role_id,
            timestamp,
        },
    )?;

    let role = database.get_role(role_id).await?;
    let history = database.get_history(user_id, role_id).await?;

    let answer = ai.chat_once(&role.prompt(), &text, Some(&history)).await?;

    let cleaned_answer = remove_brackets(&answer);
    let audio_data = reciter
        .tts(&cleaned_answer, role.gender, role.voice_type)
        .await?;
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

    if let Err(e) = summarizer.check_and_trigger(user_id, role_id).await {
        tracing::warn!("Summarizer trigger failed: {}", e);
    }

    Ok(())
}

#[derive(Deserialize)]
pub struct MessageData {
    pub user_id: i32,
    pub role_id: i32,
    pub timestamp: i64,
    pub text: String,
}

#[derive(Serialize)]
pub struct EmitData {
    pub role_id: i32,
    pub timestamp: i64,
    pub text: String,
    pub voice_url: String,
}

#[derive(Serialize)]
pub struct UserMessageSavedData {
    pub user_id: i32,
    pub role_id: i32,
    pub timestamp: i64,
}
