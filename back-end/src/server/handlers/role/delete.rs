use crate::{database::Database, error::HttpResult};
use axum::{Extension, extract::Path};
use std::sync::Arc;

pub const PATH: &str = "/api/role/delete/{role_id}/{user_id}";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Path((role_id, user_id)): Path<(i32, i32)>,
) -> HttpResult<()> {
    let deleted = database.delete_role(role_id, user_id).await?;

    if !deleted {
        return Err(anyhow::anyhow!(
            "no permission to delete this role or the role does not exist"
        )
        .into());
    }

    Ok(())
}
