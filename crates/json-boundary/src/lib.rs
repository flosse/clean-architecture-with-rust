use std::result;

use serde::{Deserialize, Serialize};

mod status_code;
pub use self::status_code::StatusCode;

pub mod domain;
pub mod usecase;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error<T> {
    /// Short error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,

    /// HTTP status code
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub status: StatusCode,
}

pub type Result<T, D> = result::Result<Response<T>, Error<D>>;
