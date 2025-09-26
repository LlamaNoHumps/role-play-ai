use crate::{
    database::Database,
    error::{HttpError, HttpResult},
    server::auth::Auth,
};
use anyhow::anyhow;
use axum::{Extension, Json};
use axum_auth::AuthBearer;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/user/profile";

#[axum::debug_handler]
pub async fn get_handler(
    Extension(auth): Extension<Auth>,
    AuthBearer(token): AuthBearer,
) -> HttpResult<Json<ProfileData>> {
    let user = auth.verify(&token).await.map_err(HttpError::Unauthorized)?;

    Ok(Json(ProfileData {
        username: user.username,
        avatar: Some(user.image),
    }))
}

#[axum::debug_handler]
pub async fn put_handler(
    Extension(auth): Extension<Auth>,
    Extension(database): Extension<Arc<Database>>,
    AuthBearer(token): AuthBearer,
    Json(payload): Json<UpdateRequestData>,
) -> HttpResult<()> {
    let user = auth.verify(&token).await.map_err(HttpError::Unauthorized)?;

    if payload.current_password != user.password_hash {
        return Err(HttpError::Unauthorized(anyhow!("当前密码不正确")));
    }

    if !payload.new_password.trim().is_empty() {
        database
            .update_user_password(user.id, &payload.new_password)
            .await?;
    }

    Ok(())
}

#[derive(Serialize)]
pub struct ProfileData {
    username: String,
    avatar: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateRequestData {
    new_password: String,
    current_password: String,
}
