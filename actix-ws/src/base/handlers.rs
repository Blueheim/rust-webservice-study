use actix_web::HttpResponse;
use common::InfoPayload;
use errors::AppError;

pub async fn check_health() -> Result<HttpResponse, AppError> {
    let message = "[actix-ws] Instance of Actix-web server is running".into();
    Ok(HttpResponse::Ok().json(InfoPayload { message }))
}
