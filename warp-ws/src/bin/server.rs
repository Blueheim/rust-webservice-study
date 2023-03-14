use errors::AppError;
use setup::{config::app_config::DataMode, APP_CONFIG};
use std::env;
use warp_ws::start;

use domains::data_source::DataSource;

// IO-bound asynchronous runtime
// Multithreaded by default
#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Load .env file
    let env_file = concat!(env!("CARGO_MANIFEST_DIR"), "/.env");
    dotenv::from_path(env_file).ok();

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
    start(data_source, addr).await?;

    Ok(())
}
