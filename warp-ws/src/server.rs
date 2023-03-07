use std::sync::Arc;

use domains::data_source::DataSource;
use warp::{http::Method, Filter};

use crate::base;

/// Start HTTP server
pub async fn start(data_source: DataSource) -> Result<(), std::io::Error> {
    // Wrap our data into an Arc for multithread concurrency
    let data = Arc::new(data_source);

    let addr = setup::setup_config::CONFIG.format_server_url();

    println!("🚀 Server listening on: {}", &addr);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let root = warp::path("api");

    let base_api = base::routes::routes_config(data);

    let api = base_api
        .or(warp::any().map(|| "Not found"))
        .with(cors)
        .with(warp::log("info"));

    let routes = root.and(api);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;

    Ok(())
}
