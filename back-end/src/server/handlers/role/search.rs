use crate::{
    database::{
        Database,
        models::roles::{Gender, VoiceType},
    },
    error::HttpResult,
};
use axum::{Extension, Json, extract::Query};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/role/search";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Query(params): Query<RequestParams>,
) -> HttpResult<Json<Vec<ResponseItem>>> {
    let keyword = params.q.trim();

    if keyword.is_empty() {
        return Ok(Json(vec![]));
    }

    let roles = database.search_roles(keyword).await?;

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

#[derive(Deserialize)]
pub struct RequestParams {
    pub q: String, // 搜索关键词
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
