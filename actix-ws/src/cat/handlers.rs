use actix_web::{web, HttpResponse, Responder};

use crate::data_source::DataSource;

pub async fn get_all_cats(data: web::Data<DataSource>) -> impl Responder {
    let cats = data.cats.read().unwrap().to_vec();
    HttpResponse::Ok().json(cats)
}
