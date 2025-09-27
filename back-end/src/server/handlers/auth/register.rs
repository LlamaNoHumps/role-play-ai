use crate::{database::Database, error::HttpResult};
use axum::{Extension, Json};
use serde::Deserialize;
use std::sync::Arc;

pub const PATH: &str = "/api/auth/register";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Json(data): Json<RequestParams>,
) -> HttpResult<()> {
    if database.get_user(&data.username).await.is_ok() {
        return Err(anyhow::anyhow!("用户已存在").into());
    }

    database
        .add_user(&data.username, &data.password, &data.avatar)
        .await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct RequestParams {
    username: String,
    password: String,
    avatar: String,
}
