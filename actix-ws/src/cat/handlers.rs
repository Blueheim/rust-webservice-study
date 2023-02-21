use actix_web::{web, HttpResponse, Responder};
use domains::models::{Cat, CatId, NewCat, UpdateCat};

use crate::data_source::DataSource;

/// Fetch all cats
pub async fn fetch_all_cats(data: web::Data<DataSource>) -> impl Responder {
    let cats = data.cats.read().unwrap().to_vec();
    HttpResponse::Ok().json(cats)
}

/// Add new cat
pub async fn add_new_cat(
    new_cat: web::Json<NewCat>, // data payload
    data: web::Data<DataSource>,
) -> impl Responder {
    let mut cats = data.cats.write().unwrap();
    let next_id = cats.len() + 1;
    let cat = Cat {
        id: CatId(next_id.to_string()),
        name: new_cat.name.clone(),
    };
    cats.push(cat);
    HttpResponse::Ok().json(cats.to_vec())
}

/// Modify existing cat
pub async fn modify_cat(
    data: web::Data<DataSource>,
    update_cat: web::Json<UpdateCat>,
    path: web::Path<u32>,
) -> impl Responder {
    let cat_id = path.into_inner();
    let mut cats = data.cats.write().unwrap();

    cats.clone()
        .into_iter()
        .position(|cat| cat.id.0 == cat_id.to_string())
        .map_or_else(
            || HttpResponse::NotFound().json("Cat not found"),
            |index| {
                let cat = Cat {
                    id: CatId(cat_id.to_string()),
                    name: update_cat.name.clone().unwrap(),
                };
                cats[index] = cat;
                HttpResponse::Ok().json(cats.to_vec())
            },
        )
}
