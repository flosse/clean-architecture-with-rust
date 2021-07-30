use adapter::model::view::json::Error;
use std::result;
use warp::{reply, Rejection};

pub type Result<T> = result::Result<T, Rejection>;

pub fn reply_error(err: Error) -> reply::WithStatus<reply::Json> {
    let status = err.status;
    let reply = reply::json(&err);
    reply::with_status(reply, status)
}
