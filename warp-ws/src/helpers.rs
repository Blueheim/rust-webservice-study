use std::sync::Arc;

use domains::data_source::DataSource;
use serde::de::DeserializeOwned;
use warp::Filter;

pub fn with_data(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (Arc<DataSource>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || data.clone())
}

pub fn json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json::<T>())
}
