use std::{sync::Arc, time::Duration};

use crate::{
    agents::{AI, RoleBuilder, Summarizer},
    database::Database,
    env::ENV,
    storage::StorageClient,
    trace::trace_middleware,
};
use axum::{
    Extension, Router, middleware,
    routing::{get, post},
};
use socketioxide::{SocketIo, extract::SocketRef};
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

    let ai = AI::new(&env.qiniu_ai_api_key);
    let role_builder = RoleBuilder::new(ai.clone());

    let (socketio_layer, socketio) = SocketIo::builder()
        .ping_interval(Duration::from_secs(3))
        .ping_timeout(Duration::from_secs(2))
        .build_layer();

    socketio.ns("/", |s: SocketRef| {});

    let router = Router::new()
        .nest_service("/static", ServeDir::new("./static"))
        .route(handlers::index::PATH, get(handlers::index::handler))
        .route(handlers::login::PATH, post(handlers::login::handler))
        .route(handlers::register::PATH, post(handlers::register::handler))
        .route(handlers::upload::PATH, post(handlers::upload::handler))
        .route(
            handlers::role::create::PATH,
            post(handlers::role::create::handler),
        )
        .route(
            handlers::role::generate::PATH,
            post(handlers::role::generate::handler),
        )
        .route(
            handlers::role::details::PATH,
            get(handlers::role::details::handler),
        )
        .route(
            handlers::role::list::PATH,
            get(handlers::role::list::handler),
        )
        .route(
            handlers::conversation::new::PATH,
            post(handlers::conversation::new::handler),
        )
        .route(
            handlers::conversation::list::PATH,
            get(handlers::conversation::list::handler),
        )
        .route(
            handlers::conversation::dialogs::PATH,
            get(handlers::conversation::dialogs::handler),
        )
        .layer(middleware::from_fn(trace_middleware))
        .layer(storage_client.into_layer())
        .layer(database.into_layer())
        .layer(ai.into_layer())
        .layer(role_builder.into_layer())
        .layer(socketio_layer)
        .layer(Extension(Arc::new(socketio)));

    let listener = tokio::net::TcpListener::bind((HOST, port)).await.unwrap();

    tracing::info!("listening on http://{}:{}", HOST, port);

    axum::serve(listener, router).await.unwrap();
}
