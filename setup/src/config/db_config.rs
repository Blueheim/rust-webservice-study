use std::env;

use clap::Parser;
use serde::Deserialize;

use crate::helpers;

pub const DEFAULT_USER: &str = "blueheim";
pub const DEFAULT_PASSWORD: &str = "dev";
pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_PORT: &str = "5432";
pub const DEFAULT_NAME: &str = "wsstudy";

#[derive(Debug, Parser, Deserialize, Default, PartialEq)]
pub struct DbConfig {
    /// Database user
    #[clap(long, default_value = DEFAULT_USER)]
    pub user: String,
    /// Database password
    #[clap(long, default_value = DEFAULT_PASSWORD)]
    pub password: String,
    /// Database host
    #[clap(long, default_value = DEFAULT_HOST)]
    pub host: String,
    /// PORT number for the database connection
    #[clap(long, default_value = DEFAULT_PORT)]
    pub port: u16,
    /// Database name
    #[clap(long, default_value = DEFAULT_NAME)]
    pub name: String,
}

impl DbConfig {
    pub fn from_env_var() -> Self {
        let user = env::var("DATABASE_USER").expect("DATABASE_USER env variable not set");
        let password =
            env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD env variable not set");
        let host = env::var("DATABASE_HOST").expect("DATABASE_HOST env variable not set");
        let port = env::var("DATABASE_PORT").expect("DATABASE_PORT env variable not set");
        let name = env::var("DATABASE_NAME").expect("DATABASE_NAME env variable not set");

        Self {
            user,
            password,
            host,
            port: port
                .parse::<u16>()
                .expect("DATABASE_PORT does not contain a valid number (u16)"),
            name,
        }
    }

    pub fn from_file() -> Self {
        let config_map = helpers::read_config_file();
        let user = config_map
            .get("database_user")
            .ok_or_else(|| "database_user is not set")
            .map_err(|err| panic!("{err}"));
        let password = config_map
            .get("database_password")
            .ok_or_else(|| "database_password is not set")
            .map_err(|err| panic!("{err}"));
        let host = config_map
            .get("database_host")
            .ok_or_else(|| "database_host is not set")
            .map_err(|err| panic!("{err}"));
        let port = config_map
            .get("database_port")
            .ok_or_else(|| "database_port is not set")
            .map_err(|err| panic!("{err}"));
        let name = config_map
            .get("database_name")
            .ok_or_else(|| "database_name is not set")
            .map_err(|err| panic!("{err}"));

        Self {
            user: user.unwrap().to_owned(),
            password: password.unwrap().to_owned(),
            host: host.unwrap().to_owned(),
            port: port
                .unwrap()
                .clone()
                .parse::<u16>()
                .expect("database_port does not contain a valid number (u16)"),
            name: name.unwrap().to_owned(),
        }
    }

    pub fn from_command_line() -> Self {
        let config = Self::parse();

        config
    }

    pub fn format_postgres_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}
