use crate::{
    agents::{AI, Reciter, Recorder, RoleBuilder, Summarizer},
    database::Database,
    env::ENV,
    storage::StorageClient,
    trace::trace_middleware,
};
use axum::{
    Extension, Router,
    extract::DefaultBodyLimit,
    middleware,
    routing::{delete, get, post},
};
use socketioxide::{SocketIo, extract::SocketRef};
use std::{sync::Arc, time::Duration};
use tower_http::services::ServeDir;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod handlers;
mod sockets;

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

    let (socketio_layer, socketio) = SocketIo::builder()
        .ping_interval(Duration::from_secs(3))
        .ping_timeout(Duration::from_secs(2))
        .build_layer();

    let ai = AI::new(&env.qiniu_ai_api_key);

    let storage_client = Arc::new(storage_client);
    let database = Arc::new(database);

    let reciter = Reciter::new(storage_client.clone(), &env.qiniu_ai_api_key);
    let role_builder = RoleBuilder::new(ai.clone(), Some(socketio.clone()), reciter.clone());

    let role_builder = Arc::new(role_builder);
    let socketio = Arc::new(socketio);
    let auth = auth::Auth::new(database.clone());

    let recorder = Recorder::new(&env.qiniu_ai_api_key);
    let database_s = database.clone();
    let summarizer = Arc::new(Summarizer::new(ai.clone(), database.clone()));
    let ai = Arc::new(ai);
    socketio.ns("/", |s: SocketRef| {
        sockets::connect(&s);
        s.on_disconnect(sockets::disconnect);
        s.on(sockets::join::EVENT, sockets::join::handler);
        s.on(sockets::message::EVENT, sockets::message::handler);
        s.on(sockets::voice::EVENT, sockets::voice::handler);
        s.extensions.insert(database_s);
        s.extensions.insert(ai);
        s.extensions.insert(reciter);
        s.extensions.insert(recorder);
        s.extensions.insert(summarizer);
    });

    let router = Router::new()
        .nest_service("/static", ServeDir::new("./static"))
        .route(handlers::index::PATH, get(handlers::index::handler))
        .route(
            handlers::auth::login::PATH,
            post(handlers::auth::login::handler),
        )
        .route(
            handlers::auth::register::PATH,
            post(handlers::auth::register::handler),
        )
        .route(
            handlers::auth::verify::PATH,
            get(handlers::auth::verify::handler),
        )
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
            handlers::role::search::PATH,
            get(handlers::role::search::handler),
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
        .route(
            handlers::conversation::delete::PATH,
            post(handlers::conversation::delete::handler),
        )
        .route(
            handlers::user::avatar::PATH,
            post(handlers::user::avatar::handler),
        )
        .route(
            handlers::user::profile::PATH,
            get(handlers::user::profile::get_handler).put(handlers::user::profile::put_handler),
        )
        .route(
            handlers::user::conversations::PATH,
            delete(handlers::user::conversations::handler),
        )
        .route(
            handlers::user::roles::LIST_PATH,
            get(handlers::user::roles::list_handler),
        )
        .route(
            handlers::user::roles::DELETE_PATH,
            delete(handlers::user::roles::delete_handler),
        )
        .layer(middleware::from_fn(trace_middleware))
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024))
        .layer(Extension(storage_client))
        .layer(Extension(database))
        .layer(Extension(auth))
        .layer(Extension(role_builder))
        .layer(socketio_layer)
        .layer(Extension(socketio));

    let listener = tokio::net::TcpListener::bind((HOST, port)).await.unwrap();

    tracing::info!("listening on http://{}:{}", HOST, port);

    axum::serve(listener, router).await.unwrap();
}
