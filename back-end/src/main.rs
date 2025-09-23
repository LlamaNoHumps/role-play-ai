use crate::env::{ENV, Env};

mod agents;
mod database;
mod env;
mod error;
mod reciter;
mod recorder;
mod server;
mod storage;
mod trace;

#[tokio::main]
async fn main() {
    ENV.get_or_init(Env::new);

    server::run().await;
}
