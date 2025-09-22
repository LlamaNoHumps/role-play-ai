use crate::{env::ENV, trace::trace_middleware};
use axum::{
    Router, middleware,
    routing::{get, post},
};
use tower_http::services::ServeDir;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;

pub async fn run() {
    let env = ENV.get().unwrap();

    let port = env.port;
    const HOST: &str = "0.0.0.0";

    tracing_subscriber::registry()
        .with(
            EnvFilter::from_default_env()
                .add_directive(format!("back-end={}", env.tracing_level).parse().unwrap()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = Router::new()
        .nest_service("/static", ServeDir::new("./static"))
        .route(handlers::index::PATH, get(handlers::index::handler))
        .layer(middleware::from_fn(trace_middleware));

    let listener = tokio::net::TcpListener::bind((HOST, port)).await.unwrap();

    tracing::info!("listening on http://{}:{}", HOST, port);

    axum::serve(listener, router).await.unwrap();
}
