use crate::{database::Database, error::HttpResult};
use axum::{Extension, Json, extract::Query};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/conversation/dialogs";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Query(RequestParams { user_id, role_id }): Query<RequestParams>,
) -> HttpResult<Json<Vec<ResponseDataItem>>> {
    let dialogs = database.list_dialogs(user_id, role_id).await?;
    let dialogs = dialogs
        .into_iter()
        .map(|dialog| ResponseDataItem {
            is_user: dialog.is_user,
            timestamp: dialog.timestamp,
            text: dialog.text,
            voice: dialog.voice,
        })
        .collect::<Vec<ResponseDataItem>>();

    Ok(Json(dialogs))
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Serialize)]
pub struct ResponseDataItem {
    pub is_user: bool,
    pub timestamp: i64,
    pub text: String,
    pub voice: Option<String>,
}
