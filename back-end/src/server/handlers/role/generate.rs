use crate::{
    agents::{RoleBuilder, role_builder::RoleBuilt},
    database::models::roles::{Gender, VoiceType},
    error::HttpResult,
};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/role/auto-fill";

#[axum::debug_handler]
pub async fn handler(
    Extension(role_builder): Extension<Arc<RoleBuilder>>,
    Json(RequestParams {
        name,
        description,
        traits,
        gender,
        voice_type,
        sid,
    }): Json<RequestParams>,
) -> HttpResult<Json<ResponseData>> {
    let description = description.unwrap_or_default();
    let traits = traits.unwrap_or_default();

    let gender = match gender {
        Some(gender) => gender.to_string(),
        None => String::new(),
    };

    let voice_type = match voice_type {
        Some(voice_type) => voice_type.to_string(),
        None => String::new(),
    };

    let RoleBuilt {
        description,
        traits,
        gender,
        voice_type,
    } = role_builder
        .build(
            &name,
            &description,
            &traits,
            &gender,
            &voice_type,
            Some(sid),
        )
        .await?;

    Ok(Json(ResponseData {
        description,
        traits,
        gender,
        voice_type,
    }))
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub name: String,
    pub description: Option<String>,
    pub traits: Option<String>,
    pub gender: Option<Gender>,
    pub voice_type: Option<VoiceType>,
    pub sid: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    pub description: String,
    pub traits: String,
    pub gender: Gender,
    pub voice_type: VoiceType,
}
