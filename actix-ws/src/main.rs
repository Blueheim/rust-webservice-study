use domains::DataSource;
use dotenv;
use std::env;
use std::io;

mod server;

mod cat;

/// Actix HTTP server
/// uses multi-threading concurrency by starting multiple worker threads on startup
/// Each thread runs a separate instance of the Actix web application

/// In addition to multi-threading, Actix uses Async I/O
/// This enables an Actix web application to perform other tasks while waiting on I/O on a single thread
/// Actix has its own Async runtime that is based on Tokio
#[actix_web::main]
async fn main() -> io::Result<()> {
    // Load .env file
    dotenv::dotenv().ok();

    // Init logger
    env_logger::init();

    // Data source definition
    let data_source = if let Ok(_) = env::var("MOCK") {
        DataSource::mock(None)
    } else {
        DataSource::db().await
    };

    // Start server app
    server::start(data_source).await
}
