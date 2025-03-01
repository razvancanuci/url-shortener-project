use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(PartialEq, Error, Debug)]
pub enum ApiError {
    NotFound(&'static str),
    BadRequest(&'static str),
    InternalServerError,
}

impl Display for ApiError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            ApiError::BadRequest(message) => _f.write_str(message),
            ApiError::NotFound(message) => _f.write_str(message),
            _ => _f.write_str("Something went wrong"),
        }
    }
}
