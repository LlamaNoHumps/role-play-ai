use crate::{database::Database, env::ENV, storage::StorageClient, trace::trace_middleware};
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

    let mut storage_client = StorageClient::new(&env.qiniu_access_key, &env.qiniu_secret_key);
    storage_client.init_bucket().await.unwrap();

    let database = Database::new(
        &env.mysql_username,
        &env.mysql_password,
        &env.mysql_endpoint,
    )
    .await
    .unwrap();
    database.init().await.unwrap();

    let router = Router::new()
        .nest_service("/static", ServeDir::new("./static"))
        .route(handlers::index::PATH, get(handlers::index::handler))
        .route(handlers::login::PATH, post(handlers::login::handler))
        .route(handlers::signup::PATH, post(handlers::signup::handler))
        .route(handlers::upload::PATH, post(handlers::upload::handler))
        .route(handlers::download::PATH, get(handlers::download::handler))
        .layer(middleware::from_fn(trace_middleware))
        .layer(storage_client.into_layer())
        .layer(database.into_layer());

    let listener = tokio::net::TcpListener::bind((HOST, port)).await.unwrap();

    tracing::info!("listening on http://{}:{}", HOST, port);

    axum::serve(listener, router).await.unwrap();
}
