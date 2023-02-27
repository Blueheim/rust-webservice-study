use std::io;

use actix_web::{
    middleware::{self, Logger, NormalizePath},
    web, App, HttpServer,
};
use domains::data_source::DataSource;

use crate::{auth, base, cat};

const ADDR: &str = "127.0.0.1:3000";

/// Start HTTP server
pub async fn start(data_source: DataSource) -> io::Result<()> {
    // web::Data will wrap our data into an Arc
    let data = web::Data::new(data_source);

    println!("🚀 Server listening on: {}", ADDR);

    // HttpServer constructs an application instance for each thread
    HttpServer::new(move || {
        let logger = Logger::default();
        let path_normalizer = NormalizePath::new(middleware::TrailingSlash::Always);

        App::new()
            .app_data(data.clone())
            .wrap(path_normalizer)
            .wrap(logger)
            // .app_data(web::JsonConfig::default().error_handler(|err, _req| {
            //     error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
            // }))
            .service(
                web::scope("/api")
                    .configure(base::routes::routes)
                    .configure(auth::routes::routes)
                    .configure(cat::routes::routes),
            )
    })
    .bind(ADDR)?
    .run()
    .await
}
