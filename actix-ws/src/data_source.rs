use std::sync::RwLock;

use domains::models::Cat;

pub struct DataSource {
    pub cats: RwLock<Vec<Cat>>,
}
