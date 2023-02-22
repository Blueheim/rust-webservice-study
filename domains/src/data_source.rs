use std::sync::RwLock;

use setup::DBStore;

use crate::models::Cat;

#[derive(Debug)]
pub enum SourceType {
    Mock(MockSource),
    DB(DBSource),
}

#[derive(Debug)]
pub struct DataSource {
    pub source: SourceType,
}

impl DataSource {
    pub fn mock(data: Option<Vec<Cat>>) -> Self {
        Self {
            source: match data {
                Some(d) => SourceType::Mock(MockSource::from(d)),
                None => SourceType::Mock(MockSource::new()),
            },
        }
    }
    pub async fn db() -> Self {
        Self {
            source: SourceType::DB(DBSource::new().await),
        }
    }
}

#[derive(Debug)]
pub struct MockSource {
    pub cats: RwLock<Vec<Cat>>,
}

impl MockSource {
    pub fn new() -> Self {
        MockSource {
            cats: RwLock::new(Cat::mock_data()),
        }
    }
    pub fn from(cats: Vec<Cat>) -> Self {
        MockSource {
            cats: RwLock::new(cats),
        }
    }
}

#[derive(Debug)]
pub struct DBSource {
    pub db: DBStore,
}

impl DBSource {
    pub async fn new() -> Self {
        DBSource {
            db: setup::create_postgres_store().await,
        }
    }
}
