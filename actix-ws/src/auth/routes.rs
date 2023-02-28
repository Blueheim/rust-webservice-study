use actix_web::web::{self, ServiceConfig};

use super::handlers;

pub const SCOPE: &str = "/auth";

// routes
pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope(SCOPE)
            .route("/signup/", web::post().to(handlers::sign_up))
            .route("/signin/", web::post().to(handlers::sign_in)),
        // .route("/signout/", web::post().to(handlers::sign_out)),
    );
}
