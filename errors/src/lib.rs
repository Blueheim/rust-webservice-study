use std::borrow::Cow;

use actix_web::{error, http::StatusCode, HttpResponse};
use common::ErrorPayload;
use derive_more::{Display, Error};
use serde::Serialize;
use validation_error_messages::PASSWORD_BAD_FORMAT;
use validator::{ValidationErrors, ValidationErrorsKind};

pub mod api_error_messages {
    pub const PASSWORD_CONFIRMATION_MISMATCH: &str = "Password and confirmation don't match";
    pub const ACCOUNT_ALREADY_EXISTING: &str = "Account already existing for that email";
    pub const EMAIL_PASSWORD_INVALID: &str = "Invalid email or password";
    pub const AUTH_TOKEN_NOT_FOUND: &str =
        "Auth token not found. Please sign in before accessing this resource";
}

pub mod validation_error_messages {
    pub const PASSWORD_BAD_FORMAT: &str = "Password must have 8 characters minimum, contains at least one uppercase letter, at least one lowercase letter, at least one number and at least one punctuation character";
    pub const PASSWORD_CONFIRMATION_MISMATCH: &str =
        "Password and confirmation password don't match";
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

#[derive(Debug, Serialize, Display)]
enum FieldErrors {
    #[display(fmt = "The field '{}' must have a minimum length of {}.", field, min)]
    MinLength { field: String, min: u64 },
    #[display(fmt = "The field '{}' must have a maximum length of {}.", field, max)]
    MaxLength { field: String, max: u64 },
    #[display(
        fmt = "The field '{}' must have a length between {} and {}.",
        field,
        min,
        max
    )]
    RangeLength { field: String, min: u64, max: u64 },
    #[display(
        fmt = "The fields '{}' and '{}' must have the same value.",
        field1,
        field2
    )]
    Equality { field1: String, field2: String },
    #[display(
        fmt = "The field '{}' doesn't have the complexity required: {}.",
        field,
        complexity_message
    )]
    Complexity {
        field: String,
        complexity_message: String,
    },
    #[display(fmt = "The field '{}' is not a valid email address.", field)]
    Email { field: String },
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let payload = match &self.error {
            Errors::Client(ClientError::InvalidBody { errors }) => {
                let errors = errors.errors().iter();
                let mut return_errors: Vec<String> = vec![];
                for (field, err) in errors {
                    match err {
                        ValidationErrorsKind::Field(field_errors) => {
                            for field_error in field_errors {
                                match field_error.code {
                                    Cow::Borrowed("must_match") => {
                                        return_errors.push(
                                            FieldErrors::Equality {
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
                                                FieldErrors::RangeLength {
                                                    field: field.to_string(),
                                                    min: min.as_u64().unwrap(),
                                                    max: max.as_u64().unwrap(),
                                                }
                                                .to_string()
                                            } else {
                                                FieldErrors::MinLength {
                                                    field: field.to_string(),
                                                    min: min.as_u64().unwrap(),
                                                }
                                                .to_string()
                                            }
                                        } else {
                                            FieldErrors::MaxLength {
                                                field: field.to_string(),
                                                max: max.unwrap().as_u64().unwrap(),
                                            }
                                            .to_string()
                                        };

                                        return_errors.push(err);
                                    }
                                    Cow::Borrowed("password_complexity") => {
                                        return_errors.push(
                                            FieldErrors::Complexity {
                                                field: field.to_string(),
                                                complexity_message: PASSWORD_BAD_FORMAT.to_owned(),
                                            }
                                            .to_string(),
                                        );
                                    }
                                    Cow::Borrowed("email") => {
                                        return_errors.push(
                                            FieldErrors::Email {
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
                println!("{:?}", return_errors);
                return_errors
            }
            _ => [self.to_string()].to_vec(),
        };
        HttpResponse::build(self.status_code()).json(payload)
    }

    fn status_code(&self) -> StatusCode {
        match self.error {
            Errors::Client(ClientError::ResourceNotFound { .. }) => StatusCode::NOT_FOUND,
            Errors::Client(ClientError::BadRequest { .. }) => StatusCode::BAD_REQUEST,
            Errors::Client(ClientError::Unauthorized { .. }) => StatusCode::UNAUTHORIZED,
            Errors::Client(ClientError::Conflict { .. }) => StatusCode::CONFLICT,
            Errors::Client(ClientError::InvalidId) => StatusCode::UNPROCESSABLE_ENTITY,
            Errors::Client(ClientError::InvalidBody { .. }) => StatusCode::BAD_REQUEST,
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
            error: Errors::Client(ClientError::InvalidBody { errors: err }),
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
                    reason: api_error_messages::EMAIL_PASSWORD_INVALID.into(),
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
    ResourceNotFound {
        resource_name: String,
        id: String,
    },
    #[display(fmt = "{}", reason)]
    BadRequest {
        reason: String,
    },
    #[display(fmt = "Access denied. {}", reason)]
    Unauthorized {
        reason: String,
    },
    #[display(fmt = "{}", reason)]
    Conflict {
        reason: String,
    },
    #[display(fmt = "Invalid Id provided.")]
    InvalidId,
    InvalidBody {
        errors: ValidationErrors,
    },
}

#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display(fmt = "Internal error. Try again later.")]
    Internal,
}
