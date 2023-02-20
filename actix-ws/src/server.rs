use std::io;

use actix_web::HttpServer;

use crate::app::WebServiceApp;

/// Start HTTP server
pub async fn start(app: WebServiceApp) -> io::Result<()> {
    // HttpServer constructs an application instance for each thread
    HttpServer::new(move || app.build())
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
