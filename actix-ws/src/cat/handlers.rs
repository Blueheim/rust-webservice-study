use actix_web::{web, HttpResponse};
use domains::{
    controller_db, controller_mock,
    models::{NewCat, ReplaceCat, UpdateCat},
    DataSource, SourceType,
};
use errors::AppError;

/// Fetch all cats
pub async fn fetch_all_cats(data: web::Data<DataSource>) -> Result<HttpResponse, AppError> {
    match &data.source {
        SourceType::Mock(data_source) => {
            let cats = controller_mock::select_all(data_source);
            Ok(HttpResponse::Ok().json(cats))
        }
        SourceType::DB(data_source) => {
            let cats = controller_db::select_all(data_source).await?;
            Ok(HttpResponse::Ok().json(cats))
        }
    }
}

/// Fetch one cat
pub async fn fetch_one_cat(
    data: web::Data<DataSource>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let cat_id = path.into_inner();

    match &data.source {
        SourceType::Mock(data_source) => {
            let cat = controller_mock::select_one(cat_id, data_source)?;
            Ok(HttpResponse::Ok().json(cat))
        }
        SourceType::DB(data_source) => {
            let cat = controller_db::select_one(cat_id, data_source).await?;
            Ok(HttpResponse::Ok().json(cat))
        }
    }
}

/// Add new cat
pub async fn add_new_cat(
    new_cat: web::Json<NewCat>, // data payload
    data: web::Data<DataSource>,
) -> Result<HttpResponse, AppError> {
    match &data.source {
        SourceType::Mock(data_source) => {
            let cat = controller_mock::create_one(new_cat.into_inner(), data_source)?;
            Ok(HttpResponse::Ok().json(cat))
        }
        SourceType::DB(data_source) => {
            let cat = controller_db::create_one(new_cat.into_inner(), data_source).await?;
            Ok(HttpResponse::Ok().json(cat))
        }
    }
}

/// Modify existing cat
pub async fn modify_cat(
    data: web::Data<DataSource>,
    update_cat: web::Json<UpdateCat>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let cat_id = path.into_inner();

    match &data.source {
        SourceType::Mock(data_source) => {
            let cat = controller_mock::update_one(cat_id, update_cat.into_inner(), data_source)?;
            Ok(HttpResponse::Ok().json(cat))
        }
        SourceType::DB(data_source) => {
            let cat =
                controller_db::update_one(cat_id, update_cat.into_inner(), data_source).await?;
            Ok(HttpResponse::Ok().json(cat))
        }
    }
}

/// Replace existing cat
pub async fn replace_cat(
    data: web::Data<DataSource>,
    replace_cat: web::Json<ReplaceCat>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let cat_id = path.into_inner();

    match &data.source {
        SourceType::Mock(data_source) => {
            let cat = controller_mock::replace_one(cat_id, replace_cat.into_inner(), data_source)?;
            Ok(HttpResponse::Ok().json(cat))
        }
        SourceType::DB(data_source) => {
            let cat =
                controller_db::replace_one(cat_id, replace_cat.into_inner(), data_source).await?;
            Ok(HttpResponse::Ok().json(cat))
        }
    }
}

/// Delete existing cat
pub async fn remove_cat(
    data: web::Data<DataSource>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let cat_id = path.into_inner();

    match &data.source {
        SourceType::Mock(data_source) => {
            let cats = controller_mock::delete_one(cat_id, data_source)?;
            Ok(HttpResponse::Ok().json(cats))
        }
        SourceType::DB(data_source) => {
            let cats = controller_db::delete_one(cat_id, data_source).await?;
            Ok(HttpResponse::Ok().json(cats))
        }
    }
}
