use std::{borrow::Cow, io};

use actix_web::{error, http::StatusCode, HttpResponse};
use common::ErrorPayload;
use derive_more::{Display, Error};
use validator::{ValidationErrors, ValidationErrorsKind};
use warp::reject::Reject;

use crate::field_errors::FieldErrorMessages;

#[derive(Debug, Display, Clone)]
pub struct AppError {
    #[display(fmt = "{}", error)]
    pub error: Errors,
}

impl AppError {
    pub fn new(error: Errors) -> Self {
        Self { error }
    }
}

#[derive(Debug, Display, Clone)]
pub enum Errors {
    Client(ClientError),
    Server(ServerError),
}

#[derive(Debug, Display, Error, Clone)]
pub enum ServerError {
    #[display(fmt = "Internal error. Try again later.")]
    Internal,
}

#[derive(Debug, Display, Error, Clone)]
pub enum ClientError {
    #[display(fmt = "Resource: {}/{} not found.", resource_name, id)]
    ResourceNotFound {
        resource_name: String,
        id: String,
    },
    #[display(fmt = "The requested route is unknown.")]
    RouteUnknown,
    #[display(fmt = "Invalid email or password")]
    InvalidCredentials,
    #[display(fmt = "Account already existing for that email")]
    AccountAlreadyExists,
    #[display(fmt = "Can't parse json body.")]
    InvalidJson,
    #[display(fmt = "Access denied. {}", reason)]
    Unauthorized {
        reason: String,
    },
    #[display(fmt = "Auth token not found. Please sign in before accessing this resource")]
    TokenNotFound,
    #[display(fmt = "Invalid Id provided.")]
    InvalidId,
    InvalidFields {
        errors: ValidationErrors,
    },
}

impl Reject for AppError {} // warp marker trait

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
            error: Errors::Client(ClientError::InvalidFields { errors: err }),
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
                error: Errors::Client(ClientError::InvalidCredentials),
            },
            _ => AppError {
                error: Errors::Server(ServerError::Internal),
            },
        }
    }
}

impl From<io::Error> for AppError {
    fn from(_err: io::Error) -> Self {
        AppError {
            error: Errors::Server(ServerError::Internal),
        }
    }
}

pub const PASSWORD_BAD_FORMAT: &str = "Password must have 8 characters minimum, contains at least one uppercase letter, at least one lowercase letter, at least one number and at least one punctuation character";

// Actix-web specific
impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let payload = match &self.error {
            Errors::Client(ClientError::InvalidFields { errors }) => {
                let errors = errors.errors().iter();
                let mut return_errors: Vec<String> = vec![];
                for (field, err) in errors {
                    match err {
                        ValidationErrorsKind::Field(field_errors) => {
                            for field_error in field_errors {
                                match field_error.code {
                                    Cow::Borrowed("must_match") => {
                                        return_errors.push(
                                            FieldErrorMessages::Equality {
                                                field1: field.to_string(),
                                                field2: field_error
                                                    .message
                                                    .as_ref()
                                                    .unwrap()
                                                    .to_string(),
                                            }
                                            .to_string(),
                                        );
                                    }
                                    Cow::Borrowed("length") => {
                                        let min = field_error.params.get("min");
                                        let max = field_error.params.get("max");

                                        let err = if let Some(min) = min {
                                            if let Some(max) = max {
                                                FieldErrorMessages::RangeLength {
                                                    field: field.to_string(),
                                                    min: min.as_u64().unwrap(),
                                                    max: max.as_u64().unwrap(),
                                                }
                                                .to_string()
                                            } else {
                                                FieldErrorMessages::MinLength {
                                                    field: field.to_string(),
                                                    min: min.as_u64().unwrap(),
                                                }
                                                .to_string()
                                            }
                                        } else {
                                            FieldErrorMessages::MaxLength {
                                                field: field.to_string(),
                                                max: max.unwrap().as_u64().unwrap(),
                                            }
                                            .to_string()
                                        };

                                        return_errors.push(err);
                                    }
                                    Cow::Borrowed("password_complexity") => {
                                        return_errors.push(
                                            FieldErrorMessages::Complexity {
                                                field: field.to_string(),
                                                complexity_message: PASSWORD_BAD_FORMAT.to_owned(),
                                            }
                                            .to_string(),
                                        );
                                    }
                                    Cow::Borrowed("email") => {
                                        return_errors.push(
                                            FieldErrorMessages::Email {
                                                field: field.to_string(),
                                            }
                                            .to_string(),
                                        );
                                    }
                                    _ => panic!(),
                                };
                            }
                        }
                        _ => {
                            unimplemented!()
                        }
                    }
                }
                return_errors
            }
            _ => [self.to_string()].to_vec(),
        };
        HttpResponse::build(self.status_code()).json(ErrorPayload { errors: payload })
    }

    fn status_code(&self) -> StatusCode {
        match self.error {
            Errors::Client(ClientError::ResourceNotFound { .. }) => StatusCode::NOT_FOUND,
            Errors::Client(ClientError::RouteUnknown) => StatusCode::NOT_FOUND,
            Errors::Client(ClientError::InvalidCredentials) => StatusCode::BAD_REQUEST,
            Errors::Client(ClientError::InvalidJson) => StatusCode::BAD_REQUEST,
            Errors::Client(ClientError::Unauthorized { .. }) => StatusCode::UNAUTHORIZED,
            Errors::Client(ClientError::TokenNotFound) => StatusCode::UNAUTHORIZED,
            Errors::Client(ClientError::AccountAlreadyExists) => StatusCode::CONFLICT,
            Errors::Client(ClientError::InvalidId) => StatusCode::UNPROCESSABLE_ENTITY,
            Errors::Client(ClientError::InvalidFields { .. }) => StatusCode::BAD_REQUEST,
            Errors::Server(ServerError::Internal) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
