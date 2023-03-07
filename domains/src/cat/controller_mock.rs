use chrono::Utc;
use errors::{AppError, ClientError, Errors};

use crate::{
    cat::models::{Cat, CatId, NewCat, ReplaceCat, UpdateCat},
    data_source::MockSource,
};

pub async fn select_all(source: &MockSource) -> Result<Vec<Cat>, AppError> {
    Ok(source.cats.read().await.to_vec())
}

pub async fn select_one(id: i32, source: &MockSource) -> Result<Cat, AppError> {
    let cats = source.cats.read().await;

    cats.clone()
        .into_iter()
        .position(|cat| cat.id.0 == id.to_string())
        .map_or_else(
            || {
                Err(AppError::new(Errors::Client(
                    ClientError::ResourceNotFound {
                        resource_name: "cats".into(),
                        id: id.to_string(),
                    },
                )))
            },
            |index| Ok(cats[index].clone()),
        )
}

pub async fn create_one(new_cat: NewCat, source: &MockSource) -> Result<Cat, AppError> {
    let mut cats = source.cats.write().await;
    let next_id = cats.len() + 1;
    let cat = Cat {
        id: CatId(next_id.to_string()),
        name: new_cat.name.clone(),
        age: new_cat.age,
        weight: new_cat.weight,
        creation_time: Utc::now(),
    };
    cats.push(cat.clone());
    Ok(cat)
}

pub async fn update_one(
    id: i32,
    update_cat: UpdateCat,
    source: &MockSource,
) -> Result<Cat, AppError> {
    let mut cats = source.cats.write().await;

    cats.clone()
        .into_iter()
        .position(|cat| cat.id.0 == id.to_string())
        .map_or_else(
            || {
                Err(AppError::new(Errors::Client(
                    ClientError::ResourceNotFound {
                        resource_name: "cats".into(),
                        id: id.to_string(),
                    },
                )))
            },
            |index| {
                let mut current_cat = cats[index].clone();

                if update_cat.name.is_some() {
                    current_cat.name = update_cat.name.clone().unwrap();
                }

                if update_cat.age.is_some() {
                    current_cat.age = update_cat.age.unwrap();
                }

                if update_cat.weight.is_some() {
                    current_cat.weight = update_cat.weight;
                }

                cats[index] = current_cat.clone();
                Ok(current_cat)
            },
        )
}

pub async fn replace_one(
    id: i32,
    replace_cat: ReplaceCat,
    source: &MockSource,
) -> Result<Cat, AppError> {
    let mut cats = source.cats.write().await;

    cats.clone()
        .into_iter()
        .position(|cat| cat.id.0 == id.to_string())
        .map_or_else(
            || {
                Err(AppError::new(Errors::Client(
                    ClientError::ResourceNotFound {
                        resource_name: "cats".into(),
                        id: id.to_string(),
                    },
                )))
            },
            |index| {
                let cat = Cat {
                    id: CatId(id.to_string()),
                    name: replace_cat.name.clone(),
                    age: replace_cat.age,
                    weight: replace_cat.weight,
                    creation_time: cats[index].creation_time,
                };
                cats[index] = cat.clone();
                Ok(cat)
            },
        )
}

pub async fn delete_one(id: i32, source: &MockSource) -> Result<String, AppError> {
    let mut cats = source.cats.write().await;

    cats.clone()
        .into_iter()
        .position(|cat| cat.id.0 == id.to_string())
        .map_or_else(
            || {
                Err(AppError::new(Errors::Client(
                    ClientError::ResourceNotFound {
                        resource_name: "cats".into(),
                        id: id.to_string(),
                    },
                )))
            },
            |index| {
                cats.remove(index);
                Ok("1 row deleted".to_string())
            },
        )
}
