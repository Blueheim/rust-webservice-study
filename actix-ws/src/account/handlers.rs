use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use domains::{account::controller_mock, data_source::DataSource};
use errors::AppError;
use setup::SuccessPayload;

use crate::middlewares::auth::JwtMiddleware;

pub async fn fetch_auth_account(
    req: HttpRequest,
    data: web::Data<DataSource>,
    _: JwtMiddleware,
) -> Result<HttpResponse, AppError> {
    let ext = req.extensions();
    let account_id = ext.get::<uuid::Uuid>().unwrap();

    let account = data
        .exec_controller(
            |data_source| {
                Box::pin(controller_mock::select_one(
                    account_id.to_owned(),
                    data_source,
                ))
            },
            |data_source| unimplemented!(),
        )
        .await?;

    Ok(HttpResponse::Ok().json(SuccessPayload { data: account }))
}
