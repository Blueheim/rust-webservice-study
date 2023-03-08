use std::sync::Arc;

use domains::{
    cat::models::{NewCat, ReplaceCat, UpdateCat},
    data_source::DataSource,
};

use warp::{Filter, Rejection, Reply};

use crate::helpers::{json_body, with_data};

use super::handlers;

pub fn routes_config(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let root = warp::path("cats");
    root.and(
        get_all(data.clone())
            .or(get_one(data.clone()))
            .or(post_one(data.clone()))
            .or(patch_one(data.clone()))
            .or(put_one(data.clone()))
            .or(delete_one(data.clone())),
    )
}

pub fn get_all(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!()
        .and(warp::get())
        .and(warp::any().map(move || data.clone()))
        .and_then(handlers::fetch_all)
}

pub fn get_one(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!(i32)
        .and(warp::get())
        .and(warp::any().map(move || data.clone()))
        .and_then(handlers::fetch_one)
}

pub fn post_one(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!()
        .and(warp::post())
        .and(with_data(data))
        .and(json_body::<NewCat>())
        .and_then(handlers::add_one)
}

pub fn patch_one(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!(i32)
        .and(warp::post())
        .and(with_data(data))
        .and(json_body::<UpdateCat>())
        .and_then(handlers::modify_one)
}

pub fn put_one(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!(i32)
        .and(warp::post())
        .and(with_data(data))
        .and(json_body::<ReplaceCat>())
        .and_then(handlers::replace_one)
}

pub fn delete_one(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!(i32)
        .and(warp::post())
        .and(with_data(data))
        .and_then(handlers::remove_one)
}
