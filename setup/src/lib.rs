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
