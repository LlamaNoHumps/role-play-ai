use crate::{auth::create_token, database::Database, error::HttpError};
use axum::{
    Extension, Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/auth/login";

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
                let user = database.get_user(&data.username).await.unwrap();

                match create_token(user.id, user.username.clone(), &user.jwt_secret) {
                    Ok(token) => (
                        StatusCode::OK,
                        Json(ResponseData {
                            user_id: user.id,
                            username: user.username,
                            image_url: user.image,
                            token,
                        }),
                    )
                        .into_response(),
                    Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create token")
                        .into_response(),
                }
            } else {
                (StatusCode::UNAUTHORIZED, "Invalid username or password").into_response()
            }
        }
        Err(e) => HttpError::from(e).into_response(),
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
