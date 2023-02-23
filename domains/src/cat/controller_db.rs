use errors::AppError;

use crate::{
    models::{Cat, CatId},
    DBSource,
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
