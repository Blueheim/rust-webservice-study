#[macro_use]
extern crate lazy_static;

mod app_config;
mod db_store;
mod jwt;

pub use db_store::*;
pub use jwt::*;

pub mod setup_config {
    use crate::app_config::AppConfig;

    lazy_static! {
        pub static ref CONFIG: AppConfig = AppConfig::read_local_config();
    }
}

pub use setup_config::*;

pub async fn create_postgres_store() -> DBStore {
    // TODO: impl on the struct instead of individual functions
    let db_url = setup_config::CONFIG.format_pg_db_url();
    let db_store = DBStore::new_postgres(&db_url).await.unwrap();

    db_store
}
