use actix_web::{web, HttpResponse};
use common::{InfoPayload, SuccessPayload};
use domains::{
    cat::{
        controller_db, controller_mock,
        models::{NewCat, ReplaceCat, UpdateCat},
    },
    data_source::DataSource,
};
use errors::AppError;

/// Fetch all cats
pub async fn fetch_all(data: web::Data<DataSource>) -> Result<HttpResponse, AppError> {
    let cats = data
        .exec_controller(
            |data_source| Box::pin(controller_mock::select_all(data_source)),
            |data_source| Box::pin(controller_db::select_all(data_source)),
        )
        .await?;

    Ok(HttpResponse::Ok().json(SuccessPayload { data: cats }))
}

/// Fetch one cat
pub async fn fetch_one(
    data: web::Data<DataSource>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let cat_id = path.into_inner();

    let cat = data
        .exec_controller(
            |data_source| Box::pin(controller_mock::select_one(cat_id, data_source)),
            |data_source| Box::pin(controller_db::select_one(cat_id, data_source)),
        )
        .await?;

    Ok(HttpResponse::Ok().json(SuccessPayload { data: cat }))
}

/// Add new cat
pub async fn add_one(
    new_cat: web::Json<NewCat>, // data payload
    data: web::Data<DataSource>,
) -> Result<HttpResponse, AppError> {
    let cat = data
        .exec_controller(
            |data_source| Box::pin(controller_mock::create_one(new_cat.clone(), data_source)),
            |data_source| Box::pin(controller_db::create_one(new_cat.clone(), data_source)),
        )
        .await?;

    Ok(HttpResponse::Ok().json(SuccessPayload { data: cat }))
}

/// Modify existing cat
pub async fn modify_one(
    data: web::Data<DataSource>,
    update_cat: web::Json<UpdateCat>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let cat_id = path.into_inner();

    let cat = data
        .exec_controller(
            |data_source| {
                Box::pin(controller_mock::update_one(
                    cat_id,
                    update_cat.clone(),
                    data_source,
                ))
            },
            |data_source| {
                Box::pin(controller_db::update_one(
                    cat_id,
                    update_cat.clone(),
                    data_source,
                ))
            },
        )
        .await?;

    Ok(HttpResponse::Ok().json(SuccessPayload { data: cat }))
}

/// Replace existing cat
pub async fn replace_one(
    data: web::Data<DataSource>,
    replace_cat: web::Json<ReplaceCat>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let cat_id = path.into_inner();

    let cat = data
        .exec_controller(
            |data_source| {
                Box::pin(controller_mock::replace_one(
                    cat_id,
                    replace_cat.clone(),
                    data_source,
                ))
            },
            |data_source| {
                Box::pin(controller_db::replace_one(
                    cat_id,
                    replace_cat.clone(),
                    data_source,
                ))
            },
        )
        .await?;

    Ok(HttpResponse::Ok().json(SuccessPayload { data: cat }))
}

/// Delete existing cat
pub async fn remove_one(
    data: web::Data<DataSource>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let cat_id = path.into_inner();

    let result = data
        .exec_controller(
            |data_source| Box::pin(controller_mock::delete_one(cat_id, data_source)),
            |data_source| Box::pin(controller_db::delete_one(cat_id, data_source)),
        )
        .await?;

    Ok(HttpResponse::Ok().json(InfoPayload { message: result }))
}
