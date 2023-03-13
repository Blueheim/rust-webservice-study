use errors::AppError;
use setup::APP_CONFIG;
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
    let data_source = if env::var("MOCK_DATA").is_ok() {
        println!("📄 Data source set to mock");
        DataSource::mock(None)
    } else {
        println!("🛢️ Data source set to db");
        DataSource::db().await
    };

    let addr = &APP_CONFIG.server.format_url();

    // Start server-app
    start(data_source, addr).await?;

    Ok(())
}
