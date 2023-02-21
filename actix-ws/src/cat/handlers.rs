use actix_web::{error, web, Error, HttpResponse};
use domains::models::{Cat, CatId, NewCat, ReplaceCat, UpdateCat};

use crate::data_source::DataSource;

/// Fetch all cats
pub async fn fetch_all_cats(data: web::Data<DataSource>) -> Result<HttpResponse, Error> {
    let cats = data.cats.read().unwrap().to_vec();
    Ok(HttpResponse::Ok().json(cats))
}

/// Fetch one cat
pub async fn fetch_one_cat(
    data: web::Data<DataSource>,
    path: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let cat_id = path.into_inner();
    let cats = data.cats.read().unwrap();

    cats.clone()
        .into_iter()
        .position(|cat| cat.id.0 == cat_id.to_string())
        .map_or_else(
            || Err(error::ErrorNotFound("Cat not found")),
            |index| Ok(HttpResponse::Ok().json(cats[index].clone())),
        )
}

/// Add new cat
pub async fn add_new_cat(
    new_cat: web::Json<NewCat>, // data payload
    data: web::Data<DataSource>,
) -> Result<HttpResponse, Error> {
    let mut cats = data.cats.write().unwrap();
    let next_id = cats.len() + 1;
    let cat = Cat {
        id: CatId(next_id.to_string()),
        name: new_cat.name.clone(),
        weight: new_cat.weight,
    };
    cats.push(cat.clone());
    Ok(HttpResponse::Ok().json(cat))
}

/// Modify existing cat
pub async fn modify_cat(
    data: web::Data<DataSource>,
    update_cat: web::Json<UpdateCat>,
    path: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let cat_id = path.into_inner();
    let mut cats = data.cats.write().unwrap();

    cats.clone()
        .into_iter()
        .position(|cat| cat.id.0 == cat_id.to_string())
        .map_or_else(
            || Err(error::ErrorNotFound("Cat not found")),
            |index| {
                let mut current_cat = cats[index].clone();

                if update_cat.name.is_some() {
                    current_cat.name = update_cat.name.clone().unwrap();
                }

                if update_cat.weight.is_some() {
                    current_cat.weight = update_cat.weight;
                }

                cats[index] = current_cat.clone();
                Ok(HttpResponse::Ok().json(current_cat))
            },
        )
}

/// Replace existing cat
pub async fn replace_cat(
    data: web::Data<DataSource>,
    update_cat: web::Json<ReplaceCat>,
    path: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let cat_id = path.into_inner();
    let mut cats = data.cats.write().unwrap();

    cats.clone()
        .into_iter()
        .position(|cat| cat.id.0 == cat_id.to_string())
        .map_or_else(
            || Err(error::ErrorNotFound("Cat not found")),
            |index| {
                let cat = Cat {
                    id: CatId(cat_id.to_string()),
                    name: update_cat.name.clone(),
                    weight: update_cat.weight,
                };
                cats[index] = cat.clone();
                Ok(HttpResponse::Ok().json(cat))
            },
        )
}

/// Delete existing cat
pub async fn remove_cat(
    data: web::Data<DataSource>,
    path: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let cat_id = path.into_inner();
    let mut cats = data.cats.write().unwrap();

    cats.clone()
        .into_iter()
        .position(|cat| cat.id.0 == cat_id.to_string())
        .map_or_else(
            || Err(error::ErrorNotFound("Cat not found")),
            |index| {
                cats.remove(index);
                Ok(HttpResponse::Ok().json(cats.to_vec()))
            },
        )
}
