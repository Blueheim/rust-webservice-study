use std::{future::Future, pin::Pin, sync::RwLock};

use errors::AppError;
use setup::DbStore;

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
    pub fn mock(data: Option<MockSource>) -> Self {
        Self {
            source: match data {
                Some(d) => SourceType::Mock(d),
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

pub enum MockData {
    Cat(Vec<Cat>),
    Account(Vec<Account>),
}

#[derive(Debug, Default)]
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
    pub fn set(mut self, data: MockData) -> Self {
        match data {
            MockData::Cat(d) => self.cats = RwLock::new(d),
            MockData::Account(d) => self.accounts = RwLock::new(d),
        }
        self
    }
}

#[derive(Debug)]
pub struct DbSource {
    pub db: DbStore,
}

impl DbSource {
    pub async fn new() -> Self {
        DbSource {
            db: DbStore::create_postgres_store().await,
        }
    }
}
