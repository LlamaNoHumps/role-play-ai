use crate::{database::Database, error::HttpResult};
use axum::{Extension, Json};
use serde::Deserialize;
use std::sync::Arc;

pub const PATH: &str = "/signup";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Json(data): Json<RequestParams>,
) -> HttpResult<()> {
    database
        .add_user(&data.username, &data.password_hash)
        .await?;

    Ok(())
}

#[derive(Deserialize)]
pub struct RequestParams {
    #[serde(rename = "userName")]
    username: String,
    #[serde(rename = "password")]
    password_hash: String,
}
