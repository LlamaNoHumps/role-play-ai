use crate::{
    agents::{RoleBuilder, role_builder::RoleBuilt},
    database::models::roles::{AgeGroup, Gender},
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
        age_group,
        sid,
    }): Json<RequestParams>,
) -> HttpResult<Json<ResponseData>> {
    let description = description.unwrap_or_default();
    let traits = traits.unwrap_or_default();

    let gender = match gender {
        Some(gender) => gender.to_string(),
        None => String::new(),
    };

    let age_group = match age_group {
        Some(age_group) => age_group.to_string(),
        None => String::new(),
    };

    let RoleBuilt {
        description,
        traits,
        gender,
        age_group,
        voice_type,
    } = role_builder
        .build(&name, &description, &traits, &gender, &age_group, Some(sid))
        .await?;

    Ok(Json(ResponseData {
        description,
        traits,
        gender,
        age_group,
        voice_type,
    }))
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub name: String,
    pub description: Option<String>,
    pub traits: Option<String>,
    pub gender: Option<Gender>,
    pub age_group: Option<AgeGroup>,
    pub sid: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    pub description: String,
    pub traits: String,
    pub gender: Gender,
    pub age_group: AgeGroup,
    pub voice_type: String,
}
