use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cat {
    pub id: CatId,
    pub name: String,
}

// Newtype idiom types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatId(String);

impl Cat {
    pub fn mock_data() -> Vec<Self> {
        let file = include_str!("cats.json");
        serde_json::from_str(file).expect("can't read cats.json")
    }
}
