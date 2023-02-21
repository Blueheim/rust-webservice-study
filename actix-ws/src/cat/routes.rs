use actix_web::web::{self, ServiceConfig};

use super::handlers::{
    add_new_cat, fetch_all_cats, fetch_one_cat, modify_cat, remove_cat, replace_cat,
};

pub const SCOPE: &str = "/cats";

// routes
pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope(SCOPE)
            .route("/", web::get().to(fetch_all_cats))
            .route("/{cat_id}/", web::get().to(fetch_one_cat))
            .route("/", web::post().to(add_new_cat))
            .route("/{cat_id}/", web::patch().to(modify_cat))
            .route("/{cat_id}/", web::put().to(replace_cat))
            .route("/{cat_id}/", web::delete().to(remove_cat)),
    );
}
