use actix_web::{error, http::StatusCode, HttpResponse};
use common::ErrorPayload;
use derive_more::{Display, Error};

pub mod messages {
    pub const PASSWORD_CONFIRMATION_MISMATCH: &str = "Password and confirmation don't match";
    pub const ACCOUNT_ALREADY_EXISTING: &str = "Account already existing for that email";
    pub const EMAIL_PASSWORD_INVALID: &str = "Invalid email or password";
    pub const AUTH_TOKEN_NOT_FOUND: &str =
        "Auth token not found. Please sign in before accessing this resource";
}

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
        HttpResponse::build(self.status_code()).json(ErrorPayload {
            error: self.to_string(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match self.error {
            Errors::Client(ClientError::ResourceNotFound { .. }) => StatusCode::NOT_FOUND,
            Errors::Client(ClientError::BadRequest { .. }) => StatusCode::BAD_REQUEST,
            Errors::Client(ClientError::Unauthorized { .. }) => StatusCode::UNAUTHORIZED,
            Errors::Client(ClientError::Conflict { .. }) => StatusCode::CONFLICT,
            Errors::Client(ClientError::InvalidId) => StatusCode::UNPROCESSABLE_ENTITY,
            Errors::Server(ServerError::Internal) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<uuid::Error> for AppError {
    fn from(_err: uuid::Error) -> Self {
        AppError {
            error: Errors::Client(ClientError::InvalidId),
        }
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError {
            error: Errors::Client(ClientError::BadRequest {
                reason: err.to_string(),
            }),
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
impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(_err: jsonwebtoken::errors::Error) -> Self {
        AppError {
            error: Errors::Server(ServerError::Internal),
        }
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        match err {
            argon2::password_hash::Error::Password => AppError {
                error: Errors::Client(ClientError::BadRequest {
                    reason: messages::EMAIL_PASSWORD_INVALID.into(),
                }),
            },
            _ => AppError {
                error: Errors::Server(ServerError::Internal),
            },
        }
    }
}

#[derive(Debug, Display, Error)]
pub enum ClientError {
    #[display(fmt = "Resource: {}/{} not found.", resource_name, id)]
    ResourceNotFound { resource_name: String, id: String },
    #[display(fmt = "{}", reason)]
    BadRequest { reason: String },
    #[display(fmt = "Access denied. {}", reason)]
    Unauthorized { reason: String },
    #[display(fmt = "{}", reason)]
    Conflict { reason: String },
    #[display(fmt = "Invalid Id provided.")]
    InvalidId,
}

#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display(fmt = "Internal error. Try again later.")]
    Internal,
}
