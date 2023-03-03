#![warn(clippy::all)]

use domains::data_source::DataSource;
use dotenv;
use std::env;
use std::io;

mod server;

mod account;
mod auth;
mod base;
mod cat;
mod middlewares;

/// Actix HTTP server
/// uses multi-threading concurrency by starting multiple worker threads on startup
/// Each thread runs a separate instance of the Actix web application

/// In addition to multi-threading, Actix uses Async I/O
/// This enables an Actix web application to perform other tasks while waiting on I/O on a single thread
/// Actix has its own Async runtime that is based on Tokio
#[actix_web::main]
async fn main() -> io::Result<()> {
    // Load .env file
    let env_file = concat!(env!("CARGO_MANIFEST_DIR"), "/.env");
    dotenv::from_path(env_file).ok();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "actix_web=info");
    }

    // Init logger
    env_logger::init();

    // Data source definition
    let data_source = if let Ok(_) = env::var("MOCK_DATA") {
        println!("📄 Data source set to mock");
        DataSource::mock(None)
    } else {
        println!("🛢️ Data source set to db");
        DataSource::db().await
    };

    // Start server app
    server::start(data_source).await
}
