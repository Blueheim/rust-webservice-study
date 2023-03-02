use actix_web::{
    cookie::{time::Duration, Cookie},
    web, HttpResponse,
};
use common::{AuthPayload, SuccessPayload};
use domains::{
    auth::{
        controller_mock,
        models::{SignInAuth, SignUpAuth},
    },
    data_source::{DataSource, SourceType},
};
use errors::AppError;
use serde_json::json;

use crate::middlewares::auth::JwtMiddleware;

pub async fn sign_up(
    auth: web::Json<SignUpAuth>,
    data: web::Data<DataSource>,
) -> Result<HttpResponse, AppError> {
    let account = data
        .exec_controller(
            |data_source| Box::pin(controller_mock::sign_up(auth.clone(), data_source)),
            |data_source| unimplemented!(),
        )
        .await?;

    Ok(HttpResponse::Ok().json(SuccessPayload {
        data: account.secure(),
    }))
}

pub async fn sign_in(
    auth: web::Json<SignInAuth>,
    data: web::Data<DataSource>,
) -> Result<HttpResponse, AppError> {
    let token = data
        .exec_controller(
            |data_source| Box::pin(controller_mock::sign_in(auth.clone(), data_source)),
            |data_source| unimplemented!(),
        )
        .await?;

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(Duration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(AuthPayload { token: Some(token) }))
}

pub async fn sign_out(_: JwtMiddleware) -> Result<HttpResponse, AppError> {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(Duration::new(-1, 0))
        .http_only(true)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(AuthPayload { token: None }))
}
