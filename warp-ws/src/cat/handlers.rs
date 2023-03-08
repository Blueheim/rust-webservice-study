use std::sync::Arc;

use common::{InfoPayload, SuccessPayload};
use domains::{
    cat::{
        controller_db, controller_mock,
        models::{NewCat, ReplaceCat, UpdateCat},
    },
    data_source::DataSource,
};
use warp::{Rejection, Reply};

pub async fn fetch_all(data: Arc<DataSource>) -> Result<impl Reply, Rejection> {
    match data
        .exec_controller(
            |data_source| Box::pin(controller_mock::select_all(data_source)),
            |data_source| Box::pin(controller_db::select_all(data_source)),
        )
        .await
    {
        Ok(cats) => Ok(warp::reply::json(&SuccessPayload { data: cats })),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn fetch_one(cat_id: i32, data: Arc<DataSource>) -> Result<impl Reply, Rejection> {
    match data
        .exec_controller(
            |data_source| Box::pin(controller_mock::select_one(cat_id, data_source)),
            |data_source| Box::pin(controller_db::select_one(cat_id, data_source)),
        )
        .await
    {
        Ok(cat) => Ok(warp::reply::json(&SuccessPayload { data: cat })),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

/// Add new cat
pub async fn add_one(data: Arc<DataSource>, new_cat: NewCat) -> Result<impl Reply, Rejection> {
    match data
        .exec_controller(
            |data_source| Box::pin(controller_mock::create_one(new_cat.clone(), data_source)),
            |data_source| Box::pin(controller_db::create_one(new_cat.clone(), data_source)),
        )
        .await
    {
        Ok(cat) => Ok(warp::reply::json(&SuccessPayload { data: cat })),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

/// Modify existing cat
pub async fn modify_one(
    cat_id: i32,
    data: Arc<DataSource>,
    update_cat: UpdateCat,
) -> Result<impl Reply, Rejection> {
    match data
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
        .await
    {
        Ok(cat) => Ok(warp::reply::json(&SuccessPayload { data: cat })),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

/// Replace existing cat
pub async fn replace_one(
    cat_id: i32,
    data: Arc<DataSource>,
    replace_cat: ReplaceCat,
) -> Result<impl Reply, Rejection> {
    match data
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
        .await
    {
        Ok(cat) => Ok(warp::reply::json(&SuccessPayload { data: cat })),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

/// Delete existing cat
pub async fn remove_one(cat_id: i32, data: Arc<DataSource>) -> Result<impl Reply, Rejection> {
    match data
        .exec_controller(
            |data_source| Box::pin(controller_mock::delete_one(cat_id, data_source)),
            |data_source| Box::pin(controller_db::delete_one(cat_id, data_source)),
        )
        .await
    {
        Ok(result) => Ok(warp::reply::json(&InfoPayload { message: result })),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
