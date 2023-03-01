use actix_web::web::{self, ServiceConfig};

use super::handlers;

pub const SCOPE: &str = "/accounts";

// routes
pub fn routes_config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope(SCOPE).route("/me/", web::get().to(handlers::fetch_auth_account)));
}
