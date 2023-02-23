use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};

#[derive(Debug, Display)]
pub enum Errors {
    Client(ClientError),
    Server(ServerError),
}

#[derive(Debug, Display)]
pub struct AppError {
    #[display(fmt = "{}", error)]
    error: Errors,
}

impl AppError {
    pub fn new(error: Errors) -> Self {
        Self { error }
    }
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self.error {
            Errors::Client(ClientError::ResourceNotFound { .. }) => StatusCode::NOT_FOUND,
            Errors::Server(ServerError::Internal) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(_err: sqlx::Error) -> Self {
        AppError {
            error: Errors::Server(ServerError::Internal),
        }
    }
}

#[derive(Debug, Display, Error)]
pub enum ClientError {
    #[display(fmt = "Resource: {}/{} not found", resource_name, id)]
    ResourceNotFound { resource_name: String, id: String },
}

#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display(fmt = "Internal error. Try again later.")]
    Internal,
}
