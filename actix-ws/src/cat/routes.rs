use actix_web::web::{self, ServiceConfig};

use super::handlers::{add_new_cat, fetch_all_cats, modify_cat};

pub const SCOPE: &str = "/cats";

// routes
pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope(SCOPE)
            .route("/", web::get().to(fetch_all_cats))
            .route("/", web::post().to(add_new_cat))
            .route("/{cat_id}/", web::patch().to(modify_cat)),
    );
}
