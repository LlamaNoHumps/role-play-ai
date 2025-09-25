use crate::{database::Database, error::HttpResult};
use axum::{Extension, extract::Path};
use std::sync::Arc;

pub const PATH: &str = "/api/conversation/delete/{user_id}/{role_id}";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Path((user_id, role_id)): Path<(i32, i32)>,
) -> HttpResult<()> {
    database.delete_conversation(user_id, role_id).await?;

    Ok(())
}
