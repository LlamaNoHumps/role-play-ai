use crate::{database::Database, error::HttpResult};
use axum::{Extension, Json};
use serde::Deserialize;
use std::sync::Arc;

pub const PATH: &str = "/api/debate/new";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Json(RequestParams {
        user_id,
        role1_id,
        role2_id,
        topic,
    }): Json<RequestParams>,
) -> HttpResult<()> {
    database
        .create_debate_table(user_id, role1_id, role2_id, &topic)
        .await?;

    Ok(())
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub user_id: i32,
    pub role1_id: i32,
    pub role2_id: i32,
    pub topic: String,
}
