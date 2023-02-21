use std::sync::RwLock;

use domains::models::Cat;

#[derive(Debug)]
pub struct DataSource {
    pub cats: RwLock<Vec<Cat>>,
}

impl DataSource {
    pub fn new() -> Self {
        Self {
            cats: RwLock::new(vec![]),
        }
    }

    pub fn from(cats: Vec<Cat>) -> Self {
        Self {
            cats: RwLock::new(cats),
        }
    }

    pub fn mock() -> Self {
        Self {
            cats: RwLock::new(Cat::mock_data()),
        }
    }
}
