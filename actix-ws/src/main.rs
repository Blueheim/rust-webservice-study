use std::{io, sync::RwLock};

use actix_web::{middleware, web, App, HttpServer};

use data_source::DataSource;
use domains::models::Cat;

mod cat;
mod data_source;

/// Actix HTTP server
/// uses multi-threading concurrency by starting multiple worker threads on startup
/// Each thread runs a separate instance of the Actix web application

/// In addition to multi-threading, Actix uses Async I/O
/// This enables an Actix web application to perform other tasks while waiting on I/O on a single thread
/// Actix has its own Async runtime that is based on Tokio
#[actix_web::main]
async fn main() -> io::Result<()> {
    // Data source definition
    let data_source = web::Data::new(DataSource {
        cats: RwLock::new(vec![
            Cat {
                name: "Kiwi".into(),
            },
            Cat {
                name: "Bella".into(),
            },
            Cat {
                name: "Gizmo".into(),
            },
        ]),
    });

    // App definition
    let app = move || {
        App::new()
            .app_data(data_source.clone())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Always,
            ))
            .service(web::scope("/api").configure(cat::routes::routes))
    };

    // Start HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
