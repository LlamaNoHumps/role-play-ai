use crate::{database::Database, error::HttpResult};
use axum::{Extension, Json, extract::Query};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const PATH: &str = "/api/conversation/dialogs";

#[axum::debug_handler]
pub async fn handler(
    Extension(database): Extension<Arc<Database>>,
    Query(params): Query<RequestParams>,
) -> HttpResult<Json<PaginatedResponse<ResponseDataItem>>> {
    let dialogs = database
        .list_dialogs_paginated(
            params.user_id,
            params.role_id,
            params.offset.unwrap_or(0),
            params.limit.unwrap_or(20),
        )
        .await?;

    let dialog_items = dialogs
        .items
        .into_iter()
        .map(|dialog| ResponseDataItem {
            is_user: dialog.is_user,
            timestamp: dialog.timestamp,
            text: dialog.text,
            voice: dialog.voice,
        })
        .collect::<Vec<ResponseDataItem>>();

    Ok(Json(PaginatedResponse {
        items: dialog_items,
        total: dialogs.total,
        has_more: dialogs.has_more,
    }))
}

#[derive(Deserialize)]
pub struct RequestParams {
    pub user_id: i32,
    pub role_id: i32,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub has_more: bool,
}

#[derive(Serialize)]
pub struct ResponseDataItem {
    pub is_user: bool,
    pub timestamp: i64,
    pub text: String,
    pub voice: Option<String>,
}
