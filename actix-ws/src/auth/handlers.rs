use actix_web::{web, HttpResponse};
use domains::{
    auth::{controller_mock, models::SignUpAuth},
    data_source::{DataSource, SourceType},
};
use errors::AppError;

pub async fn sign_up(
    auth: web::Json<SignUpAuth>,
    data: web::Data<DataSource>,
) -> Result<HttpResponse, AppError> {
    match &data.source {
        SourceType::Mock(data_source) => {
            let account = controller_mock::sign_up(auth.into_inner(), data_source)?;
            Ok(HttpResponse::Ok().json(account))
        }
        SourceType::DB(data_source) => {
            unimplemented!()
        }
    }
}
