use crate::{agents::RoleBuilder, error::HttpResult};
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
    }): Json<RequestParams>,
) -> HttpResult<Json<ResponseData>> {
    let description = description.unwrap_or_default();
    let traits = traits.unwrap_or_default();

    let (description, traits) = role_builder.build(&name, &description, &traits).await?;

    Ok(Json(ResponseData {
        description,
        traits,
    }))
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub name: String,
    pub description: Option<String>,
    pub traits: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseData {
    pub description: String,
    pub traits: String,
}
