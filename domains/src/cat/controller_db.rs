use errors::AppError;

use crate::{
    cat::models::{Cat, CatId, CurrentCat, NewCat, ReplaceCat, UpdateCat},
    data_source::DBSource,
};

pub async fn select_all(source: &DBSource) -> Result<Vec<Cat>, AppError> {
    let cats: Vec<Cat> = sqlx::query!("SELECT * FROM cats")
        .map(|row| Cat {
            id: CatId(row.id.to_string()),
            name: row.name,
            age: row.age,
            weight: row.weight,
            creation_time: row.created_on,
        })
        .fetch_all(&source.db.connection)
        .await?;

    Ok(cats)
}

pub async fn select_one(id: i32, source: &DBSource) -> Result<Cat, AppError> {
    let cat: Cat = sqlx::query!("SELECT * FROM cats WHERE id = $1", id)
        .map(|row| Cat {
            id: CatId(row.id.to_string()),
            name: row.name,
            age: row.age,
            weight: row.weight,
            creation_time: row.created_on,
        })
        .fetch_one(&source.db.connection)
        .await?;

    Ok(cat)
}

pub async fn create_one(new_cat: NewCat, source: &DBSource) -> Result<Cat, AppError> {
    let cat: Cat = sqlx::query!(
        "INSERT INTO cats (name, age, weight) 
         VALUES ($1, $2, $3) 
         RETURNING id, name, age, weight, created_on",
        new_cat.name,
        new_cat.age,
        new_cat.weight
    )
    .map(|row| Cat {
        id: CatId(row.id.to_string()),
        name: row.name,
        age: row.age,
        weight: row.weight,
        creation_time: row.created_on,
    })
    .fetch_one(&source.db.connection)
    .await?;

    Ok(cat)
}

pub async fn update_one(
    id: i32,
    update_cat: UpdateCat,
    source: &DBSource,
) -> Result<Cat, AppError> {
    // Retrieve current data
    let current_cat = sqlx::query_as!(
        CurrentCat,
        "SELECT name, age, weight FROM cats WHERE id = $1",
        id
    )
    .fetch_one(&source.db.connection)
    .await?;

    let name = match update_cat.name {
        Some(name) => name,
        None => current_cat.name,
    };

    let age = match update_cat.age {
        Some(age) => age,
        None => current_cat.age,
    };

    let weight = match update_cat.weight {
        Some(weight) => weight,
        None => current_cat.weight.unwrap_or_default(),
    };

    let cat: Cat = sqlx::query!(
        "UPDATE cats SET name = $1, age = $2, weight = $3  
         WHERE id = $4
         RETURNING id, name, age, weight, created_on",
        name,
        age,
        weight,
        id
    )
    .map(|row| Cat {
        id: CatId(row.id.to_string()),
        name: row.name,
        age: row.age,
        weight: row.weight,
        creation_time: row.created_on,
    })
    .fetch_one(&source.db.connection)
    .await?;

    Ok(cat)
}

pub async fn replace_one(
    id: i32,
    replace_cat: ReplaceCat,
    source: &DBSource,
) -> Result<Cat, AppError> {
    let cat: Cat = sqlx::query!(
        "UPDATE cats SET name = $1, age = $2, weight = $3  
         WHERE id = $4
         RETURNING id, name, age, weight, created_on",
        replace_cat.name,
        replace_cat.age,
        replace_cat.weight,
        id
    )
    .map(|row| Cat {
        id: CatId(row.id.to_string()),
        name: row.name,
        age: row.age,
        weight: row.weight,
        creation_time: row.created_on,
    })
    .fetch_one(&source.db.connection)
    .await?;

    Ok(cat)
}

pub async fn delete_one(id: i32, source: &DBSource) -> Result<String, AppError> {
    let result = sqlx::query!("DELETE FROM cats WHERE id = $1", id)
        .execute(&source.db.connection)
        .await?;

    Ok(result.rows_affected().to_string())
}
