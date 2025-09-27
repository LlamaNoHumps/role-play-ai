use crate::{database::Database, error::HttpResult};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
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
) -> HttpResult<Json<ResponseData>> {
    let debate_id = database
        .create_debate_table(user_id, role1_id, role2_id, &topic)
        .await?;

    Ok(Json(ResponseData { debate_id }))
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub user_id: i32,
    pub role1_id: i32,
    pub role2_id: i32,
    pub topic: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    pub debate_id: i32,
}
