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

pub const PATH: &str = "/api/role/list";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Query(params): Query<RequestParams>,
) -> HttpResult<Json<PaginatedResponse<ResponseItem>>> {
    let roles = database
        .list_roles_paginated(params.offset.unwrap_or(0), params.limit.unwrap_or(15))
        .await?;

    let mut response_data = Vec::new();

    for role in &roles.items {
        response_data.push(ResponseItem {
            role_id: role.id,
            name: role.name.clone(),
            description: role.description.clone(),
            traits: role.traits.clone(),
            image_url: role.image.clone(),
            gender: role.gender.clone(),
            voice_type: role.voice_type.clone(),
        });
    }

    Ok(Json(PaginatedResponse {
        items: response_data,
        total: roles.total,
        has_more: roles.has_more,
    }))
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub has_more: bool,
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
