use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    /// Log level
    pub log_level: String,
    /// Database user
    pub database_user: String,
    /// Database password
    pub database_password: String,
    /// Database url
    pub database_host: String,
    /// PORT number for the database connection
    pub database_port: u16,
    /// Database name
    pub database_name: String,
    /// Web server host
    pub server_host: String,
    /// Web server port
    pub server_port: u16,
    /// Jwt token
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn read_local_config() -> Self {
        let config = Config::builder()
            .add_source(config::File::with_name("setup/config"))
            .build()
            .unwrap();

        let config = config.try_deserialize::<Self>().unwrap();

        config
    }

    pub fn format_server_url(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port,)
    }

    pub fn format_pg_db_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database_user,
            self.database_password,
            self.database_host,
            self.database_port,
            self.database_name
        )
    }
}
