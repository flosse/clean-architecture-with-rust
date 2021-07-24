#[cfg(test)]
use serde::Deserialize;
use serde::Serialize;
use std::result;
use warp::{http::StatusCode, reply, Rejection};

#[derive(Serialize)]
#[cfg_attr(test, derive(Deserialize))]
pub struct JsonError {
    pub msg: String,
    pub status_code: u16,
}

pub struct Error {
    pub msg: String,
    pub status: StatusCode,
}

impl From<Error> for JsonError {
    fn from(err: Error) -> Self {
        Self {
            msg: err.msg,
            status_code: err.status.into(),
        }
    }
}

pub type Result<T> = result::Result<T, Rejection>;

pub fn reply_error(err: Error) -> reply::WithStatus<reply::Json> {
    let status = err.status;
    let json_err = JsonError::from(err);
    let reply = reply::json(&json_err);
    reply::with_status(reply, status)
}

mod thought {
    use super::Error;
    use application::gateway::repository::thought as repository;
    use warp::http::StatusCode;

    impl From<repository::Error> for Error {
        fn from(err: repository::Error) -> Self {
            use repository::Error as E;
            match err {
                E::NotFound => Self {
                    msg: "Could not find thought".to_string(),
                    status: StatusCode::NOT_FOUND,
                },
                E::Io(_) => Self {
                    msg: "A database error occured".to_string(),
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                },
            }
        }
    }
    mod find_by_id {

        use super::Error;
        use adapter::controller::thought::find_by_id as controller;
        use application::usecase::thought::find_by_id as usecase;
        use warp::http::StatusCode;

        impl From<controller::Error> for Error {
            fn from(err: controller::Error) -> Self {
                use controller::Error as E;

                match err {
                    E::Parameter(err) => Self {
                        msg: err.to_string(),
                        status: StatusCode::BAD_REQUEST,
                    },
                    E::Usecase(usecase::Error::Repo(err)) => err.into(),
                }
            }
        }
    }
    mod create {
        use super::Error;
        use adapter::controller::thought::create as controller;
        use application::usecase::thought::create as usecase;
        use warp::http::StatusCode;

        impl From<controller::Error> for Error {
            fn from(err: controller::Error) -> Self {
                use controller::Error as E;

                match err {
                    E::Usecase(usecase::Error::Invalidity(err)) => Self {
                        msg: err.to_string(),
                        status: StatusCode::BAD_REQUEST,
                    },
                    E::Usecase(usecase::Error::Repo(err)) => err.into(),
                }
            }
        }
    }
}
