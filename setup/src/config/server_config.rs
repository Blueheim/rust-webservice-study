use std::env;

use clap::Parser;
use serde::Deserialize;

use crate::helpers;

#[derive(Debug, Parser, Deserialize, Default)]
#[clap(author, version, about, long_about = None)]
/// Web server structure study
pub struct ServerConfig {
    /// Log level
    /// Decide which kind of errors we want to log (info, warn, error)
    #[clap(long, default_value = "info")]
    pub log_level: String,
    /// Web server ip addr host
    #[clap(long, default_value = "127.0.0.1")]
    pub host_ip: String,
    /// Web server port
    #[clap(long, default_value = "3000", value_parser = clap::value_parser!(u16).range(1..))]
    pub port: u16,
}

impl ServerConfig {
    pub fn from_file() -> Self {
        let config_map = helpers::read_config_file();
        let log_level = config_map
            .get("log_level")
            .ok_or_else(|| "log_level is not set")
            .map_err(|err| panic!("{err}"));
        let host_ip = config_map
            .get("server_host_ip")
            .ok_or_else(|| "server_host_ip is not set")
            .map_err(|err| panic!("{err}"));
        let port = config_map
            .get("server_port")
            .ok_or_else(|| "server_port is not set")
            .map_err(|err| panic!("{err}"));

        Self {
            log_level: log_level.unwrap().to_owned(),
            host_ip: host_ip.unwrap().to_owned(),
            port: port
                .unwrap()
                .to_owned()
                .parse::<u16>()
                .expect("server_port does not contain a valid number (u16)"),
        }
    }

    pub fn from_command_line() -> Self {
        let config = Self::parse();

        config
    }

    pub fn from_env_var() -> Self {
        let log_level = env::var("LOG_LEVEL").expect("LOG_LEVEL env variable not set");
        let host_ip = env::var("SERVER_HOST_IP").expect("SERVER_HOST_IP env variable not set");
        let port = env::var("SERVER_PORT").expect("SERVER_PORT env variable not set");

        Self {
            log_level,
            host_ip,
            port: port
                .parse::<u16>()
                .expect("SERVER_PORT does not contain a valid number (u16)"),
        }
    }

    pub fn format_url(&self) -> String {
        format!("{}:{}", self.host_ip, self.port,)
    }
}
