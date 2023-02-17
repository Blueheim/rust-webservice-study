use std::io;

use actix_web::{
    web::{self, ServiceConfig},
    App, HttpResponse, HttpServer, Responder,
};

// routes
pub fn routes(cfg: &mut ServiceConfig) {
    cfg.route("/", web::get().to(handler));
}

// handler
pub async fn handler() -> impl Responder {
    HttpResponse::Ok().json("Hello from actix !")
}

/// Actix HTTP server
/// uses multi-threading concurrency by starting multiple worker threads on startup
/// Each thread runs a separate instance of the Actix web application

/// In addition to multi-threading, Actix uses Async I/O
/// This enables an Actix web application to perform other tasks while waiting on I/O on a single thread
/// Actix has its own Async runtime that is based on Tokio
#[actix_web::main]
async fn main() -> io::Result<()> {
    // App definition
    let app = move || App::new().configure(routes);

    // Start HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
