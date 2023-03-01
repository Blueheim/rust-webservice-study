use actix_web::web;

use super::handlers;

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/health/", web::get().to(handlers::check_health));
}
