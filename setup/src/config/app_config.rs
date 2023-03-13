use errors::AppError;
use std::{env, str::FromStr};

use super::{db_config::DbConfig, server_config::ServerConfig};

#[derive(Debug)]
pub struct AppConfig {
    /// Server instance configuration
    pub server: ServerConfig,
    /// Database configuration
    pub database: DbConfig,
    /// Server and database config source (file, command, both, env variables)
    pub config_source: ConfigSource,
    /// env mode (development or production)
    pub env_mode: EnvMode,
    /// data mode (file or database)
    pub data_mode: DataMode,
}

#[derive(Debug, PartialEq)]
pub enum ConfigSource {
    File,
    CommandLine,
    Both,
    EnvVar,
}

impl FromStr for ConfigSource {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "file" => Ok(Self::File),
            "command_line" => Ok(Self::CommandLine),
            "both" => Ok(Self::Both),
            "env_var" => Ok(Self::EnvVar),
            _invalid_source => panic!("Invalid config source"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum EnvMode {
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
pub enum DataMode {
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
            return Ok(Self {
                server: server_config,
                database: db_config,
                config_source: ConfigSource::EnvVar,
                env_mode,
                data_mode,
            });
        }

        // Development Mode
        let server_config;
        let mut db_config = DbConfig::default();
        match config_source {
            ConfigSource::File => {
                server_config = ServerConfig::from_file();
                if data_mode == DataMode::Database {
                    db_config = DbConfig::from_file();
                }
            }
            ConfigSource::CommandLine => {
                server_config = ServerConfig::from_command_line();
                if data_mode == DataMode::Database {
                    db_config = DbConfig::from_command_line();
                }
            }
            ConfigSource::Both => {
                unimplemented!()
            }
            ConfigSource::EnvVar => {
                server_config = ServerConfig::from_env_var();
                if data_mode == DataMode::Database {
                    db_config = DbConfig::from_env_var();
                }
            }
        }
        let app_config = Self {
            server: server_config,
            database: db_config,
            config_source,
            env_mode,
            data_mode,
        };

        println!("{:?}", app_config);
        Ok(app_config)
    }
}
