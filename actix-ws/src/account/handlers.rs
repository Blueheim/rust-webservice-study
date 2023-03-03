use actix_web::{web, HttpResponse};
use common::SuccessPayload;
use domains::{account::controller_mock, data_source::DataSource};
use errors::AppError;

use crate::middlewares::auth::JwtMiddleware;

pub async fn fetch_auth_account(
    data: web::Data<DataSource>,
    jwt: JwtMiddleware,
) -> Result<HttpResponse, AppError> {
    let account_id = jwt.account_id;

    let account = data
        .exec_controller(
            |data_source| {
                Box::pin(controller_mock::select_one(
                    account_id.to_owned(),
                    data_source,
                ))
            },
            |_data_source| unimplemented!(),
        )
        .await?;

    Ok(HttpResponse::Ok().json(SuccessPayload { data: account }))
}
