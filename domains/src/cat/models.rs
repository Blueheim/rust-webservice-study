use serde::{Deserialize, Serialize};

// Newtype idiom types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatId(pub String);

/// Cat struct
/// Mostly to be serialized from db record to json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cat {
    pub id: CatId,
    pub name: String,
    pub weight: Option<u32>,
}

impl Cat {
    pub fn mock_data() -> Vec<Self> {
        let file = include_str!("cats.json");
        serde_json::from_str(file).expect("can't read cats.json")
    }
}

/// New Cat struct
/// Mostly to be deserialized from json to db record
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewCat {
    pub name: String,
    pub weight: Option<u32>,
}

/// Update Cat struct
/// Mostly to be deserialized from json to db record
/// All the fields are optional
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCat {
    pub name: Option<String>,
    pub weight: Option<u32>,
}

/// Replace Cat struct
/// Mostly to be deserialized from json to db record
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReplaceCat {
    pub name: String,
    pub weight: Option<u32>,
}
