use std::result;

use serde::Serialize;
use warp::{reply, Rejection};

use cawr_adapter::model::view::json::{Error, StatusCode};

pub type Result<T> = result::Result<T, Rejection>;

pub fn reply_error<T>(err: Error<T>) -> reply::WithStatus<reply::Json>
where
    T: serde::Serialize,
{
    let status = into_warp_status_code(err.status);
    let reply = reply::json(&err);
    reply::with_status(reply, status)
}

pub fn reply_json<T>(data: &T, status: StatusCode) -> reply::WithStatus<reply::Json>
where
    T: Serialize,
{
    let json = reply::json(data);
    let status = into_warp_status_code(status);
    reply::with_status(json, status)
}

fn into_warp_status_code(status: StatusCode) -> warp::http::StatusCode {
    // This must never fail because `StatusCode::as_u16` should return a valid code.
    warp::http::StatusCode::from_u16(status.as_u16()).expect("HTTP status code")
}
