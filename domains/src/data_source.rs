use std::{
    future::{Future, IntoFuture},
    pin::Pin,
    sync::RwLock,
};

use errors::AppError;
use setup::DBStore;

use crate::{account::models::Account, cat::models::Cat};

#[derive(Debug)]
pub enum SourceType {
    Mock(MockSource),
    DB(DbSource),
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
            source: SourceType::DB(DbSource::new().await),
        }
    }
    pub fn exec_controller<'a, T, M, N>(
        &'a self,
        mock_fn: M,
        db_fn: N,
    ) -> Pin<Box<(dyn Future<Output = Result<T, AppError>> + 'a)>>
    where
        M: Fn(&'a MockSource) -> Pin<Box<(dyn Future<Output = Result<T, AppError>> + 'a)>>,
        N: Fn(&'a DbSource) -> Pin<Box<(dyn Future<Output = Result<T, AppError>> + 'a)>>,
    {
        match &self.source {
            SourceType::Mock(data_source) => mock_fn(data_source),
            SourceType::DB(data_source) => db_fn(data_source),
        }
    }
}

#[derive(Debug)]
pub struct MockSource {
    pub accounts: RwLock<Vec<Account>>,
    pub cats: RwLock<Vec<Cat>>,
}

impl MockSource {
    pub fn new() -> Self {
        MockSource {
            accounts: RwLock::new(Account::mock_data()),
            cats: RwLock::new(Cat::mock_data()),
        }
    }
    pub fn from(cats: Vec<Cat>) -> Self {
        MockSource {
            accounts: RwLock::new(vec![]), // TODO
            cats: RwLock::new(cats),
        }
    }
}

#[derive(Debug)]
pub struct DbSource {
    pub db: DBStore,
}

impl DbSource {
    pub async fn new() -> Self {
        DbSource {
            db: setup::create_postgres_store().await,
        }
    }
}
