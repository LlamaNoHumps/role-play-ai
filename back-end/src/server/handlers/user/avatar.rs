use crate::{
    database::Database,
    error::{HttpError, HttpResult},
    server::auth::Auth,
};
use axum::{Extension, Json};
use axum_auth::AuthBearer;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/user/avatar";

#[axum::debug_handler]
pub async fn handler(
    Extension(auth): Extension<Auth>,
    Extension(database): Extension<Arc<Database>>,
    AuthBearer(token): AuthBearer,
    Json(payload): Json<RequestData>,
) -> HttpResult<Json<ResponseData>> {
    let user = auth.verify(&token).await.map_err(HttpError::Unauthorized)?;

    database
        .update_user_avatar(user.id, &payload.avatar_url)
        .await?;

    Ok(Json(ResponseData {
        success: true,
        message: "头像更新成功".to_string(),
    }))
}

#[derive(Deserialize)]
pub struct RequestData {
    avatar_url: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    success: bool,
    message: String,
}
