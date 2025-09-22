use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use tokio::time::Instant;

pub async fn trace_middleware(req: Request<Body>, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let version = req.version();

    let user_agent = req
        .headers()
        .get(axum::http::header::USER_AGENT)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("-")
        .to_string();

    let start = Instant::now();
    let response = next.run(req).await;
    let latency = start.elapsed();

    tracing::info!(
        "method={}, uri={}, version={:?}, latency={:?}, status={}, user-agent={}",
        method,
        uri,
        version,
        latency,
        response.status(),
        user_agent
    );

    response
}
