use crate::{
    database::Database,
    error::{HttpError, HttpResult},
    server::auth::Auth,
};
use anyhow::anyhow;
use axum::{Extension, Json, extract::Path};
use axum_auth::AuthBearer;
use serde::Serialize;
use std::sync::Arc;

pub const DELETE_PATH: &str = "/api/user/role/delete/{role_id}";
pub const LIST_PATH: &str = "/api/user/roles";

#[axum::debug_handler]
pub async fn list_handler(
    Extension(auth): Extension<Auth>,
    Extension(database): Extension<Arc<Database>>,
    AuthBearer(token): AuthBearer,
) -> HttpResult<Json<Vec<RoleData>>> {
    let user = auth.verify(&token).await.map_err(HttpError::Unauthorized)?;

    let roles = database.get_user_roles(user.id).await?;

    let role_data = roles
        .into_iter()
        .map(|role| RoleData {
            id: role.id,
            name: role.name,
            description: role.description,
            image: role.image,
            traits: role.traits,
        })
        .collect();

    Ok(Json(role_data))
}

#[axum::debug_handler]
pub async fn delete_handler(
    Extension(auth): Extension<Auth>,
    Extension(database): Extension<Arc<Database>>,
    AuthBearer(token): AuthBearer,
    Path(role_id): Path<i32>,
) -> HttpResult<()> {
    let user = auth.verify(&token).await.map_err(HttpError::Unauthorized)?;

    let role = database.get_role(role_id).await?;

    if role.user_id != user.id {
        return Err(HttpError::Forbidden(anyhow!("无权限删除此角色")));
    }

    database.delete_role_and_conversations(role_id).await?;

    Ok(())
}

#[derive(Serialize)]
pub struct RoleData {
    id: i32,
    name: String,
    description: String,
    image: String,
    traits: String,
}
