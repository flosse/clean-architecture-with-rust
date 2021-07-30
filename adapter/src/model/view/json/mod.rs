use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::result;

pub mod thought;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub msg: Option<String>,
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
}

impl Error {
    pub const fn internal() -> Self {
        Self {
            msg: None, // We really want to hide internal details
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub data: T,
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
}

pub type Result<T> = result::Result<Response<T>, Error>;
