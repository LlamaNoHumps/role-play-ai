use crate::error::HttpResult;
use axum::response::Html;
use tokio::fs::read_to_string;

pub const PATH: &str = "/";

#[axum::debug_handler]
pub async fn handler() -> HttpResult<Html<String>> {
    let html = read_to_string("./index.html").await?;

    Ok(Html(html))
}
