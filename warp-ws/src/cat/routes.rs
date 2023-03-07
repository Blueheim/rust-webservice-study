use std::sync::Arc;

use domains::data_source::DataSource;

use warp::{Filter, Rejection, Reply};

use super::handlers;

pub fn routes_config(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let root = warp::path("cats");
    root.and(get_cats(data.clone()).or(warp::any().map(|| "Not found")))
    //get_cats(data.clone())
}

pub fn get_cats(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!()
        .and(warp::get())
        .and(warp::any().map(move || data.clone()))
        .and_then(handlers::fetch_all)
}
