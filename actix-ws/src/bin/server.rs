#![warn(clippy::all)]

use actix_ws::start;
use setup::config::app_config::DataMode;
use setup::APP_CONFIG;

use domains::data_source::DataSource;

use std::env;
use std::io;

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

    let log_level = &APP_CONFIG.server.log_level;
    env::set_var("RUST_LOG", format!("actix_web={}", log_level));

    // Init logger
    env_logger::init();

    // Data source selection
    let data_source = match &APP_CONFIG.data_mode {
        DataMode::File => {
            println!("📄 Data source set to: File");
            DataSource::mock(None)
        }
        DataMode::Database => {
            println!("🛢️ Data source set to: Db");
            DataSource::db().await
        }
    };

    let addr = &APP_CONFIG.server.format_url();

    // Start server-app
    start(data_source, addr).await
}
