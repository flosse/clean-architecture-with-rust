use crate::{
    model::{
        app::thought as app,
        view::json::{thought as view, Error, Response, Result},
    },
    presenter::Present,
};
use application::{gateway::repository::thought as repo, usecase::thought as uc};
use http::StatusCode;

#[derive(Default)]
pub struct Presenter;

// -- Create -- //

impl Present<app::create::Result> for Presenter {
    type ViewModel = Result<view::ThoughtId>;
    fn present(&self, res: app::create::Result) -> Self::ViewModel {
        res.map(|data| view::ThoughtId(data.id.to_string()))
            .map(|id| Response {
                data: id,
                status: StatusCode::CREATED,
            })
            .map_err(|err| match err {
                app::create::Error::Invalidity(err) => Error {
                    msg: Some(err.to_string()),
                    status: StatusCode::BAD_REQUEST,
                },
                app::create::Error::Repo(_) => Error::internal(),
            })
    }
}

// -- Find by ID -- //

impl Present<app::find_by_id::Result> for Presenter {
    type ViewModel = Result<view::Thought>;
    fn present(&self, res: app::find_by_id::Result) -> Self::ViewModel {
        res.map(|data| {
            let uc::find_by_id::Response { id, title } = data;
            let id = id.to_string();
            view::Thought { id, title }
        })
        .map(|data| Response {
            data,
            status: StatusCode::OK,
        })
        .map_err(|err| match err {
            app::find_by_id::Error::Id(err) => Error {
                msg: Some(err.to_string()),
                status: StatusCode::BAD_REQUEST,
            },
            app::find_by_id::Error::Repo(err) => err.into(),
        })
    }
}

impl From<repo::Error> for Error {
    fn from(err: repo::Error) -> Self {
        match err {
            repo::Error::NotFound => Self {
                msg: Some("Could not find thought".to_string()),
                status: StatusCode::NOT_FOUND,
            },
            repo::Error::Io(_) => Self::internal(),
        }
    }
}
