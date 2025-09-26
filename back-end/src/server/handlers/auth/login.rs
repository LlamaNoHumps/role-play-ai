use crate::{
    database::Database,
    error::{HttpError, HttpResult},
    server::auth::Auth,
};
use anyhow::anyhow;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/auth/login";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Json(data): Json<RequestParams>,
) -> HttpResult<Json<ResponseData>> {
    if database
        .verify_user(&data.username, &data.password_hash)
        .await?
    {
        let user = database.get_user(&data.username).await.unwrap();
        let token = Auth::create_token(user.id, user.username.clone(), &user.jwt_secret)?;

        Ok(Json(ResponseData {
            user_id: user.id,
            username: user.username,
            image_url: user.image,
            token,
        }))
    } else {
        Err(HttpError::Unauthorized(anyhow!(
            "Invalid username or password"
        )))
    }
}

#[derive(Deserialize)]
pub struct RequestParams {
    #[serde(rename = "username")]
    username: String,
    #[serde(rename = "password")]
    password_hash: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    user_id: i32,
    username: String,
    #[serde(rename = "avatar")]
    image_url: String,
    token: String,
}
