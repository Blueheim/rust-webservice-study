use actix_web::{web, HttpResponse};
use domains::{
    controller_mock,
    models::{NewCat, ReplaceCat, UpdateCat},
    DataSource, SourceType,
};
use errors::AppError;

/// Fetch all cats
pub async fn fetch_all_cats(data: web::Data<DataSource>) -> Result<HttpResponse, AppError> {
    match &data.source {
        SourceType::Mock(source) => {
            let cats = controller_mock::select_all(source);
            Ok(HttpResponse::Ok().json(cats))
        }
        SourceType::DB(source) => {
            todo!()
        }
    }
}

/// Fetch one cat
pub async fn fetch_one_cat(
    data: web::Data<DataSource>,
    path: web::Path<u32>,
) -> Result<HttpResponse, AppError> {
    match &data.source {
        SourceType::Mock(source) => {
            let cat_id = path.into_inner();
            let cat = controller_mock::select_one(cat_id, source)?;
            Ok(HttpResponse::Ok().json(cat))
        }
        SourceType::DB(source) => {
            todo!()
        }
    }
}

/// Add new cat
pub async fn add_new_cat(
    new_cat: web::Json<NewCat>, // data payload
    data: web::Data<DataSource>,
) -> Result<HttpResponse, AppError> {
    match &data.source {
        SourceType::Mock(source) => {
            let cat = controller_mock::create_one(new_cat.into_inner(), source)?;
            Ok(HttpResponse::Ok().json(cat))
        }
        SourceType::DB(source) => {
            todo!()
        }
    }
}

/// Modify existing cat
pub async fn modify_cat(
    data: web::Data<DataSource>,
    update_cat: web::Json<UpdateCat>,
    path: web::Path<u32>,
) -> Result<HttpResponse, AppError> {
    match &data.source {
        SourceType::Mock(source) => {
            let cat_id = path.into_inner();
            let cat = controller_mock::update_one(cat_id, update_cat.into_inner(), source)?;
            Ok(HttpResponse::Ok().json(cat))
        }
        SourceType::DB(source) => {
            todo!()
        }
    }
}

/// Replace existing cat
pub async fn replace_cat(
    data: web::Data<DataSource>,
    replace_cat: web::Json<ReplaceCat>,
    path: web::Path<u32>,
) -> Result<HttpResponse, AppError> {
    match &data.source {
        SourceType::Mock(source) => {
            let cat_id = path.into_inner();
            let cat = controller_mock::replace_one(cat_id, replace_cat.into_inner(), source)?;
            Ok(HttpResponse::Ok().json(cat))
        }
        SourceType::DB(source) => {
            todo!()
        }
    }
}

/// Delete existing cat
pub async fn remove_cat(
    data: web::Data<DataSource>,
    path: web::Path<u32>,
) -> Result<HttpResponse, AppError> {
    match &data.source {
        SourceType::Mock(source) => {
            let cat_id = path.into_inner();
            let cats = controller_mock::delete_one(cat_id, source)?;
            Ok(HttpResponse::Ok().json(cats))
        }
        SourceType::DB(source) => {
            todo!()
        }
    }
}
