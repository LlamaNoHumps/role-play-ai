use crate::env::{ENV, Env};

mod database;
mod env;
mod error;
mod reciter;
mod recorder;
mod server;
mod storage;
mod trace;
mod writer;

#[tokio::main]
async fn main() {
    ENV.get_or_init(Env::new);

    server::run().await;
}
