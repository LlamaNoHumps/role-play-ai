use crate::{database::Database, error::HttpResult};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/role/create";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Json(RequestParams {
        name,
        description,
        traits,
        image_url,
    }): Json<RequestParams>,
) -> HttpResult<Json<ResponseData>> {
    let role_id = database
        .add_role(&name, &description, &traits, &image_url)
        .await?;

    Ok(Json(ResponseData { role_id }))
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub name: String,
    pub description: String,
    pub traits: String,
    #[serde(rename = "avatar")]
    pub image_url: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    pub role_id: i32,
}
