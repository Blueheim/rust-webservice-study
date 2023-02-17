use actix_web::web::{self, ServiceConfig};

use super::handlers::get_all_cats;

pub const SCOPE: &str = "/cats";

// routes
pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(web::scope(SCOPE).route("/", web::get().to(get_all_cats)));
}
