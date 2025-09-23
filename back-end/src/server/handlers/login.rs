use crate::{database::Database, error::HttpError};
use axum::{
    Extension, Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use std::sync::Arc;

pub const PATH: &str = "/login";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Json(data): Json<RequestParams>,
) -> Response {
    match database
        .verify_user(&data.username, &data.password_hash)
        .await
    {
        Ok(verified) => {
            if verified {
                (StatusCode::OK, "Login successful").into_response()
            } else {
                (StatusCode::UNAUTHORIZED, "Invalid username or password").into_response()
            }
        }
        Err(e) => HttpError::from(e).into_response(),
    }
}

#[derive(Deserialize)]
pub struct RequestParams {
    #[serde(rename = "userName")]
    username: String,
    #[serde(rename = "password")]
    password_hash: String,
}
