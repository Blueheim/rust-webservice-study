use errors::AppError;
use std::{env, str::FromStr};

use super::{db_config::DbConfig, server_config::ServerConfig};

#[derive(Debug, PartialEq)]
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

impl<'a> Into<&'a str> for EnvMode {
    fn into(self) -> &'a str {
        match self {
            Self::Development => "development",
            Self::Production => "production",
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

impl<'a> Into<&'a str> for DataMode {
    fn into(self) -> &'a str {
        match self {
            Self::File => "file",
            Self::Database => "Database",
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

/// WARN: Don't use parallelism when running these tests
/// These tests would interfere with each other because they use env variables
/// To execute them consecutively use --test-threads=1 (e.g cargo test -- --test-threads=1)
#[cfg(test)]
mod app_config_tests {
    use super::*;

    #[test]
    fn test_default_config() {
        // Arrange
        env::remove_var("ENV_MODE");
        env::remove_var("CONFIG_SOURCE");
        env::remove_var("DATA_MODE");
        // Act
        let config = AppConfig::new().unwrap();
        // Assert
        let expected = AppConfig {
            server: ServerConfig::from_file(),
            database: DbConfig::default(),
            config_source: DEFAULT_CONFIG_SOURCE,
            env_mode: DEFAULT_ENV_MODE,
            data_mode: DEFAULT_DATA_MODE,
        };
        assert_eq!(config, expected);
    }

    #[test]
    fn test_production_unset_server_var_fail() {
        // Arrange
        env::set_var::<&str, &str>("ENV_MODE", EnvMode::Production.into());
        env::set_var("LOG_LEVEL", "");
        env::set_var("SERVER_HOST_IP", "");
        env::set_var("SERVER_PORT", "");
        // Act
        let config = std::panic::catch_unwind(|| AppConfig::new());
        // Assert
        assert!(config.is_err());
    }

    #[test]
    fn test_production_unset_database_var_fail() {
        // Arrange
        env::set_var::<&str, &str>("ENV_MODE", EnvMode::Production.into());
        env::set_var("LOG_LEVEL", "warn");
        env::set_var("SERVER_HOST_IP", "127.0.0.1");
        env::set_var("SERVER_PORT", "4500");
        env::set_var::<&str, &str>("DATA_MODE", DataMode::Database.into());
        env::set_var("DATABASE_USER", "");
        env::set_var("DATABASE_PASSWORD", "");
        env::set_var("DATABASE_PORT", "");
        env::set_var("DATABASE_HOST", "");
        env::set_var("DATABASE_NAME", "");
        // Act
        let config = std::panic::catch_unwind(|| AppConfig::new());
        // Assert
        assert!(config.is_err());
    }

    #[test]
    fn test_production_with_db_source() {
        // Arrange
        env::set_var::<&str, &str>("ENV_MODE", EnvMode::Production.into());
        env::set_var("LOG_LEVEL", "warn");
        env::set_var("SERVER_HOST_IP", "127.0.0.1");
        env::set_var("SERVER_PORT", "4500");
        env::set_var::<&str, &str>("DATA_MODE", DataMode::Database.into());
        env::set_var("DATABASE_USER", "user");
        env::set_var("DATABASE_PASSWORD", "password");
        env::set_var("DATABASE_PORT", "6889");
        env::set_var("DATABASE_HOST", "localhost");
        env::set_var("DATABASE_NAME", "test");
        // Act
        let config = AppConfig::new().unwrap();
        // Assert
        // Assert
        let expected = AppConfig {
            server: ServerConfig::from_env_var(),
            database: DbConfig::from_env_var(),
            config_source: ConfigSource::EnvVar,
            env_mode: EnvMode::Production,
            data_mode: DataMode::Database,
        };
        assert_eq!(config, expected);
    }
}
