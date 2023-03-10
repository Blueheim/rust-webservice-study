use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::setup_config;

#[derive(Debug, Clone)]
pub struct DbStore {
    pub connection: PgPool,
}

impl DbStore {
    pub async fn create_postgres_store() -> DbStore {
        let db_url = setup_config::APP_CONFIG.database.format_postgres_url();
        let db_store = DbStore::new_postgres(&db_url).await.unwrap();

        db_store
    }
    pub async fn new_postgres(db_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = match PgPoolOptions::new()
            .max_connections(4)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        };

        Ok(DbStore {
            connection: db_pool,
        })
    }
}
