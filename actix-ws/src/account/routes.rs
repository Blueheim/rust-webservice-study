use actix_web::web::{self, ServiceConfig};

use super::handlers;

pub const SCOPE: &str = "/accounts";

// routes
pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(web::scope(SCOPE).route("/me/", web::post().to(handlers::fetch_auth_account)));
}
