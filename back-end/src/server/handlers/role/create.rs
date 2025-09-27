use crate::{
    database::{
        Database,
        models::roles::{AgeGroup, Gender},
    },
    error::HttpResult,
};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/role/create";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Json(RequestParams {
        user_id,
        name,
        description,
        traits,
        avatar,
        gender,
        age_group,
        voice_type,
    }): Json<RequestParams>,
) -> HttpResult<Json<ResponseData>> {
    let role_id = database
        .add_role(
            user_id,
            &name,
            &description,
            &traits,
            &avatar,
            gender,
            age_group,
            &voice_type,
        )
        .await?;

    Ok(Json(ResponseData { role_id }))
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub user_id: i32,
    pub name: String,
    pub description: String,
    pub traits: String,
    pub avatar: String,
    pub gender: Gender,
    pub age_group: AgeGroup,
    pub voice_type: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    pub role_id: i32,
}
