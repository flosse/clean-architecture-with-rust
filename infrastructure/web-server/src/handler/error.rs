use cawr_adapter::model::view::json::Error;
use std::result;
use warp::{reply, Rejection};

pub type Result<T> = result::Result<T, Rejection>;

pub fn reply_error<T>(err: Error<T>) -> reply::WithStatus<reply::Json>
where
    T: serde::Serialize,
{
    let status = err.status;
    let reply = reply::json(&err);
    reply::with_status(reply, status)
}
