use std::{convert::Infallible, sync::Arc};

use common::InfoPayload;
use domains::data_source::DataSource;
use warp::Reply;

pub async fn check_health(_data: Arc<DataSource>) -> Result<impl Reply, Infallible> {
    let message = "[warp-ws] Instance of Warp server is running".into();
    Ok(warp::reply::json(&InfoPayload { message }))
}
