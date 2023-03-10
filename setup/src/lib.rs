#[macro_use]
extern crate lazy_static;

mod app_config;
mod auth_config;
mod db_store;
mod helpers;

pub use db_store::*;
pub use helpers::*;

pub mod setup_config {
    use crate::{app_config::AppConfig, auth_config::AuthConfig};

    lazy_static! {
        pub static ref APP_CONFIG: AppConfig = AppConfig::new().expect("Error building app config");
        pub static ref AUTH_CONFIG: AuthConfig = AuthConfig::new();
    }
}

pub use setup_config::*;
