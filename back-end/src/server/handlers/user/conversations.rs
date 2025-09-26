use crate::{
    database::Database,
    error::{HttpError, HttpResult},
    server::auth::Auth,
};
use axum::{Extension, Json};
use axum_auth::AuthBearer;
use serde::Serialize;
use std::sync::Arc;

pub const PATH: &str = "/api/user/conversations/delete";

#[axum::debug_handler]
pub async fn handler(
    Extension(auth): Extension<Auth>,
    Extension(database): Extension<Arc<Database>>,
    AuthBearer(token): AuthBearer,
) -> HttpResult<Json<ResponseData>> {
    let user = auth.verify(&token).await.map_err(HttpError::Unauthorized)?;

    let deleted_count = database.delete_all_user_conversations(user.id).await?;

    Ok(Json(ResponseData { deleted_count }))
}

#[derive(Serialize)]
pub struct ResponseData {
    deleted_count: u64,
}
