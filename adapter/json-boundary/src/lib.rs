use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::result;

pub mod domain;
pub mod usecase;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error<T> {
    /// Short error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,

    /// HTTP status code
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,

    /// Structured error details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<T>,
}

impl<T> Error<T> {
    pub const fn internal() -> Self {
        Self {
            msg: None, // We really want to hide internal details
            status: StatusCode::INTERNAL_SERVER_ERROR,
            details: None, // We really want to hide internal details
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub data: T,
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
}

pub type Result<T, D> = result::Result<Response<T>, Error<D>>;
