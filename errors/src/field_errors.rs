use std::{borrow::Cow, io};

use actix_web::{error, http::StatusCode, HttpResponse};
use common::ErrorPayload;
use derive_more::{Display, Error};
use serde::Serialize;
use validator::{ValidationErrors, ValidationErrorsKind};
use warp::reject::Reject;

#[derive(Debug, Serialize, Display)]
pub enum FieldErrorMessages {
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
