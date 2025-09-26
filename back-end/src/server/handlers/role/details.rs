use crate::{database::Database, error::HttpResult};
use axum::{Extension, Json, extract::Query};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/role/details";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Query(RequestData { role_id }): Query<RequestData>,
) -> HttpResult<Json<ResponseData>> {
    let role = database.get_role(role_id).await?;

    Ok(Json(ResponseData {
        role_id: role.id,
        name: role.name,
        description: role.description,
        traits: role.traits,
        image_url: role.image,
    }))
}

#[derive(Deserialize)]
pub struct RequestData {
    pub role_id: i32,
}

#[derive(Serialize)]
pub struct ResponseData {
    pub role_id: i32,
    pub name: String,
    pub description: String,
    pub traits: String,
    pub image_url: String,
}
