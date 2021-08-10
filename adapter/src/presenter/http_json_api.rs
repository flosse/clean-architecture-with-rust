use crate::{
    model::{
        app::thought as app,
        view::json::{thought as view, Error, Response, Result},
    },
    presenter::Present,
};
use http::StatusCode;
use std::convert::TryFrom;

#[derive(Default)]
pub struct Presenter;

// -- Create -- //

impl Present<app::create::Result> for Presenter {
    type ViewModel = Result<view::ThoughtId, view::create::Error>;
    fn present(&self, res: app::create::Result) -> Self::ViewModel {
        res.map(view::ThoughtId::from)
            .map(|id| Response {
                data: id,
                status: StatusCode::CREATED,
            })
            .map_err(|err| match &err {
                app::create::Error::Invalidity(invalidity) => Error {
                    msg: Some(invalidity.to_string()),
                    status: StatusCode::BAD_REQUEST,
                    details: view::create::Error::try_from(err).ok(),
                },
                app::create::Error::Repo => Error::internal(),
            })
    }
}

// -- Find by ID -- //

impl Present<app::find_by_id::Result> for Presenter {
    type ViewModel = Result<view::Thought, view::find_by_id::Error>;
    fn present(&self, res: app::find_by_id::Result) -> Self::ViewModel {
        res.map(view::Thought::from)
            .map(|data| Response {
                data,
                status: StatusCode::OK,
            })
            .map_err(|err| match err {
                app::find_by_id::Error::Id => Error {
                    msg: Some(err.to_string()),
                    status: StatusCode::BAD_REQUEST,
                    details: Some(view::find_by_id::Error::Id),
                },
                app::find_by_id::Error::NotFound => Error {
                    msg: Some("Could not find thought".to_string()),
                    status: StatusCode::NOT_FOUND,
                    details: Some(view::find_by_id::Error::NotFound),
                },
                app::find_by_id::Error::Repo => Error::internal(),
            })
    }
}
