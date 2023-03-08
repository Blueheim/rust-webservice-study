use std::convert::Infallible;

use common::ErrorPayload;
use warp::body::BodyDeserializeError;
use warp::cors::CorsForbidden;

use warp::{Rejection, Reply};

use crate::{AppError, ClientError, Errors, ServerError};

// Warp specific
pub async fn handle_rejection(rejection: Rejection) -> Result<impl Reply, Infallible> {
    let payload;
    if rejection.is_not_found() {
        payload = [ClientError::RouteUnknown.to_string()].to_vec();
    } else if let Some(AppError { error }) = rejection.find() {
        payload = match error {
            Errors::Client(ClientError::InvalidFields { errors }) => {
                unimplemented!()
            }
            _ => [error.to_string()].to_vec(),
        };
    } else if let Some(_error) = rejection.find::<BodyDeserializeError>() {
        payload = [ClientError::InvalidJson.to_string()].to_vec();
    } else if let Some(error) = rejection.find::<CorsForbidden>() {
        payload = [ClientError::Forbidden {
            reason: error.to_string(),
        }
        .to_string()]
        .to_vec();
    } else {
        payload = [ServerError::Internal.to_string()].to_vec();
    }

    Ok(warp::reply::json(&ErrorPayload { errors: payload }))
}
