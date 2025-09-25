use crate::{
    database::{
        Database,
        models::roles::{Gender, VoiceType},
    },
    error::HttpResult,
};
use axum::{Extension, Json};
use serde::Serialize;
use std::sync::Arc;

pub const PATH: &str = "/api/role/list";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
) -> HttpResult<Json<Vec<ResponseItem>>> {
    let roles = database.list_roles().await?;

    let mut response_data = Vec::new();

    for role in roles {
        response_data.push(ResponseItem {
            role_id: role.id,
            name: role.name,
            description: role.description,
            traits: role.traits,
            image_url: role.image,
            gender: role.gender,
            voice_type: role.voice_type,
        });
    }

    Ok(Json(response_data))
}

#[derive(Serialize)]
pub struct ResponseItem {
    pub role_id: i32,
    pub name: String,
    pub description: String,
    pub traits: String,
    pub image_url: String,
    pub gender: Gender,
    pub voice_type: VoiceType,
}
