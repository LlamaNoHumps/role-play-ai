use crate::{
    auth::{unsafe_decode_token, verify_token},
    database::Database,
};
use axum::{
    Extension, Json,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::sync::Arc;

pub const PATH: &str = "/api/auth/verify";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    headers: HeaderMap,
) -> Response {
    let auth_header = match headers.get("Authorization") {
        Some(header) => header,
        None => return (StatusCode::UNAUTHORIZED, "Missing authorization header").into_response(),
    };

    let token = match auth_header.to_str() {
        Ok(auth_str) => {
            if auth_str.starts_with("Bearer ") {
                auth_str.strip_prefix("Bearer ").unwrap()
            } else {
                return (StatusCode::UNAUTHORIZED, "Invalid authorization format").into_response();
            }
        }
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, "Invalid authorization header").into_response();
        }
    };

    match unsafe_decode_token(token) {
        Ok(claims) => match database.get_user(&claims.username).await {
            Ok(user) => match verify_token(token, &user.jwt_secret) {
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
