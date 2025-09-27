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
    if database.verify_user(&data.username, &data.password).await? {
        let user = database.get_user(&data.username).await.unwrap();
        let token = Auth::create_token(user.id, user.username.clone(), &user.jwt_secret)?;

        Ok(Json(ResponseData {
            user_id: user.id,
            username: user.username,
            avatar: user.image,
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
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    user_id: i32,
    username: String,
    avatar: String,
    token: String,
}
