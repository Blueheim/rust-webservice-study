use std::sync::Arc;

use common::SuccessPayload;
use domains::{
    cat::{controller_db, controller_mock},
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
