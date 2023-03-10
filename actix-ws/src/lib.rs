use std::io;

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::{self, Logger, NormalizePath},
    web, App, HttpServer,
};
use domains::data_source::DataSource;
use errors::{AppError, ClientError, Errors};

mod account;
mod auth;
mod base;
mod cat;
mod middlewares;

/// Start HTTP server
pub async fn start(data_source: DataSource, addr: &str) -> io::Result<()> {
    // web::Data will wrap our data into an Arc
    let data = web::Data::new(data_source);

    println!("🚀 Server listening on: {}", addr);

    // HttpServer constructs an application instance for each thread
    HttpServer::new(move || {
        let logger = Logger::default();
        let path_normalizer = NormalizePath::new(middleware::TrailingSlash::Always);
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST", "PATCH", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(data.clone())
            .wrap(cors)
            .wrap(path_normalizer)
            .wrap(logger)
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                AppError::new(Errors::Client(ClientError::InvalidJson)).into()
            }))
            .service(
                web::scope("/api")
                    .configure(base::routes::routes_config)
                    .configure(auth::routes::routes_config)
                    .configure(account::routes::routes_config)
                    .configure(cat::routes::routes_config),
            )
    })
    .bind(addr)?
    .run()
    .await
}
