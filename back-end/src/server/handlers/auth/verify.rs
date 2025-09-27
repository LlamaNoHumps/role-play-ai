use crate::{
    error::{HttpError, HttpResult},
    server::auth::Auth,
};
use axum::{Extension, Json};
use axum_auth::AuthBearer;
use serde::Serialize;

pub const PATH: &str = "/api/auth/verify";

#[axum::debug_handler]
pub async fn handler(
    Extension(auth): Extension<Auth>,
    AuthBearer(token): AuthBearer,
) -> HttpResult<Json<ResponseData>> {
    let user = auth.verify(&token).await.map_err(HttpError::Unauthorized)?;

    Ok(Json(ResponseData {
        user_id: user.id,
        username: user.username,
        avatar: user.image,
    }))
}

#[derive(Serialize)]
pub struct ResponseData {
    user_id: i32,
    username: String,
    avatar: String,
}
