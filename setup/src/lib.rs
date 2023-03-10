#[macro_use]
extern crate lazy_static;

pub mod config;
pub mod db_store;
pub mod helpers;

pub mod setup_config {
    use crate::{config::app_config::AppConfig, config::auth_config::AuthConfig};

    lazy_static! {
        pub static ref APP_CONFIG: AppConfig = AppConfig::new().expect("Error building app config");
        pub static ref AUTH_CONFIG: AuthConfig = AuthConfig::new();
    }
}

pub use setup_config::*;
