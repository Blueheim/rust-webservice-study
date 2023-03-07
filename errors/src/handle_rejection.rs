use std::convert::Infallible;
use std::error::Error;

use common::ErrorPayload;
use serde_json::json;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::{AppError, ClientError, Errors};

// Warp specific
pub async fn handle_rejection(rej: Rejection) -> Result<impl Reply, Infallible> {
    let payload;
    if let Some(AppError { error }) = rej.find() {
        payload = match error {
            Errors::Client(ClientError::InvalidFields { errors }) => {
                unimplemented!()
            }
            _ => [error.to_string()].to_vec(),
        };
    } else {
        payload = [ClientError::RouteUnknown.to_string()].to_vec();
    }

    Ok(warp::reply::json(&ErrorPayload { errors: payload }))
}
