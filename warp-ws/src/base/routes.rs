use std::sync::Arc;

use domains::data_source::DataSource;

use warp::{Filter, Rejection, Reply};

use super::handlers;

pub fn routes_config(
    data: Arc<DataSource>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_health(data.clone())
}

pub fn get_health(
    data: Arc<DataSource>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("health")
        .and(warp::get())
        .and(warp::any().map(move || data.clone()))
        .and_then(handlers::check_health)
}
