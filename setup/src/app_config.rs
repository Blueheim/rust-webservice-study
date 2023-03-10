use clap::Parser;
use config::Config;
use errors::{AppError, ConfigError, Errors};
use lazy_static::__Deref;
use serde::Deserialize;
use std::{collections::HashMap, env, str::FromStr};

#[derive(Debug, Parser, Deserialize, Default)]
#[clap(author, version, about, long_about = None)]
/// Web server structure study
pub struct ServerConfig {
    /// Log level
    /// Decide which kind of errors we want to log (info, warn, error)
    #[clap(short, long, default_value = "info")]
    pub log_level: String,
    /// Web server ip addr host
    #[clap(short, long, default_value = "127.0.0.1")]
    pub host_ip: String,
    /// Web server port
    #[clap(short, long, default_value = "3000")]
    pub port: u16,
}

impl ServerConfig {
    pub fn from_file() -> Self {
        let config_map = read_config_file();
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

#[derive(Debug, Parser, Deserialize, Default)]
pub struct DbConfig {
    /// Database user
    #[clap(long, default_value = "blueheim")]
    pub user: String,
    /// Database password
    #[clap(long, default_value = "dev")]
    pub password: String,
    /// Database host
    #[clap(long, default_value = "127.0.0.1")]
    pub host: String,
    /// PORT number for the database connection
    #[clap(long, default_value = "5432")]
    pub port: u16,
    /// Database name
    #[clap(long, default_value = "wsstudy")]
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
        let config_map = read_config_file();
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

    pub fn format_postgres_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}

#[derive(Debug)]
pub struct AppConfig {
    /// Server instance configuration
    pub server: ServerConfig,
    /// Database configuration
    pub database: DbConfig,
    /// Server and database config source (file, command, both, env variables)
    config_source: ConfigSource,
    /// env mode (development or production)
    env_mode: EnvMode,
    /// data mode (file or database)
    data_mode: DataMode,
}

#[derive(Debug, PartialEq)]
enum ConfigSource {
    File,
    Command,
    Both,
    EnvVar,
}

impl FromStr for ConfigSource {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "file" => Ok(Self::File),
            "command" => Ok(Self::Command),
            "both" => Ok(Self::Both),
            "env_var" => Ok(Self::EnvVar),
            _invalid_source => panic!("Invalid config source"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum EnvMode {
    Development,
    Production,
}

impl FromStr for EnvMode {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            _invalid_env => panic!("Invalid env mode"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum DataMode {
    File,
    Database,
}

impl FromStr for DataMode {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "file" => Ok(Self::File),
            "database" => Ok(Self::Database),
            _invalid_data_mode => panic!("Invalid data mode"),
        }
    }
}

const DEFAULT_ENV_MODE: EnvMode = EnvMode::Development;
const DEFAULT_CONFIG_SOURCE: ConfigSource = ConfigSource::File;
const DEFAULT_DATA_MODE: DataMode = DataMode::File;

impl AppConfig {
    pub fn new() -> Result<Self, AppError> {
        let app_config;

        let env_mode = match env::var("ENV_MODE") {
            Ok(mode) => EnvMode::from_str(mode.as_str())?,
            Err(_) => DEFAULT_ENV_MODE,
        };

        let config_source = match env::var("CONFIG_SOURCE") {
            Ok(source) => ConfigSource::from_str(source.as_str())?,
            Err(_) => DEFAULT_CONFIG_SOURCE,
        };

        let data_mode = match env::var("DATA_MODE") {
            Ok(mode) => DataMode::from_str(mode.as_str())?,
            Err(_) => DEFAULT_DATA_MODE,
        };

        if env_mode == EnvMode::Production {
            // Force all configs to come from env variables
            let server_config = ServerConfig::from_env_var();
            let mut db_config = DbConfig::default();
            if data_mode == DataMode::Database {
                db_config = DbConfig::from_env_var();
            }
            app_config = Self {
                server: server_config,
                database: db_config,
                config_source: ConfigSource::EnvVar,
                env_mode,
                data_mode,
            };
        } else {
            // Development Mode
            if config_source == ConfigSource::File {
                let server_config = ServerConfig::from_file();
                let mut db_config = DbConfig::default();
                if data_mode == DataMode::Database {
                    db_config = DbConfig::from_file();
                }
                app_config = Self {
                    server: server_config,
                    database: db_config,
                    config_source,
                    env_mode,
                    data_mode,
                };
            } else if config_source == ConfigSource::Both {
                unimplemented!()
            } else {
                unimplemented!()
            }
        }

        println!("{:?}", app_config);
        return Ok(app_config);
    }
}

fn read_config_file() -> HashMap<String, String> {
    let file = concat!(env!("CARGO_MANIFEST_DIR"), "/config");
    let config = Config::builder()
        .add_source(config::File::with_name(file))
        .build()
        .unwrap();

    config.try_deserialize::<HashMap<String, String>>().unwrap()
}
