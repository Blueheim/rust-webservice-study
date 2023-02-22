use config::Config;
use serde::Deserialize;

mod db_store;

pub use db_store::*;

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
    /// Web server port
    pub port: u16,
}

fn read_local_config() -> AppConfig {
    let config = Config::builder()
        .add_source(config::File::with_name("setup"))
        .build()
        .unwrap();

    let config = config.try_deserialize::<AppConfig>().unwrap();

    config
}

fn format_pg_db_url(config: AppConfig) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database_user,
        config.database_password,
        config.database_host,
        config.database_port,
        config.database_name
    )
}

pub async fn create_postgres_store() -> DBStore {
    // TODO: impl on the struct instead of individual functions
    let config = read_local_config();
    let db_url = format_pg_db_url(config);
    let db_store = DBStore::new_postgres(&db_url).await.unwrap();

    db_store
}
