use std::{net::SocketAddrV4, sync::Arc};

use domains::data_source::DataSource;
use errors::handle_rejection;
use warp::{http::Method, Filter};

mod base;
mod cat;
mod helpers;

/// Start HTTP server
pub async fn start(data_source: DataSource, addr: &str) -> Result<(), std::io::Error> {
    // Wrap our data into an Arc for multithread concurrency
    let data = Arc::new(data_source);

    println!("🚀 Server listening on: {}", &addr);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let root_scope = warp::path("api");

    let base_api = base::routes::routes_config(data.clone());
    let cat_api = cat::routes::routes_config(data.clone());

    let api = base_api.or(cat_api).with(cors).with(warp::log("info"));

    let routes = root_scope.and(api).recover(handle_rejection);

    let socket = addr
        .parse::<SocketAddrV4>()
        .expect("Can't parse addr into v4");
    warp::serve(routes).run(socket).await;

    Ok(())
}
