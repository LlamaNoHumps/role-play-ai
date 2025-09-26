use crate::{
    auth::{unsafe_decode_token, verify_token},
    database::Database,
};
use axum::{
    Extension, Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_auth::AuthBearer;
use serde::Serialize;
use std::sync::Arc;

pub const PATH: &str = "/api/auth/verify";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    AuthBearer(token): AuthBearer,
) -> Response {
    match unsafe_decode_token(&token) {
        Ok(claims) => match database.get_user(&claims.username).await {
            Ok(user) => match verify_token(&token, &user.jwt_secret) {
                Ok(_verified_claims) => (
                    StatusCode::OK,
                    Json(ResponseData {
                        user_id: user.id,
                        username: user.username,
                        avatar: user.image,
                    }),
                )
                    .into_response(),
                Err(_) => (StatusCode::UNAUTHORIZED, "Invalid token signature").into_response(),
            },
            Err(_) => (StatusCode::UNAUTHORIZED, "User not found").into_response(),
        },
        Err(_) => (StatusCode::UNAUTHORIZED, "Invalid token format").into_response(),
    }
}

#[derive(Serialize)]
pub struct ResponseData {
    user_id: i32,
    username: String,
    avatar: String,
}
