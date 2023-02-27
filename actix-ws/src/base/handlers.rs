use actix_web::HttpResponse;
use errors::AppError;

pub async fn check_health() -> Result<HttpResponse, AppError> {
    const MESSAGE: &str = "[actix-ws] Instance of Actix-web server is running";
    Ok(HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE})))
}
