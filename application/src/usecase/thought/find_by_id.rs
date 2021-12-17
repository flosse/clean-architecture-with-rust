use crate::gateway::repository::thought::{GetError, Record, Repo};
use domain::thought::Id;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug)]
pub struct Response {
    pub id: Id,
    pub title: String,
}

impl From<Record> for Response {
    fn from(r: Record) -> Self {
        let Record { thought } = r;
        let title = String::from(thought.title);
        let id = thought.id;
        Self { id, title }
    }
}

/// Find thought by ID usecase interactor
pub struct FindById<'r, R> {
    repo: &'r R,
}

impl<'r, R> FindById<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", GetError::NotFound)]
    NotFound,
    #[error("{}", GetError::Connection)]
    Repo,
}

impl From<GetError> for Error {
    fn from(e: GetError) -> Self {
        match e {
            GetError::NotFound => Self::NotFound,
            GetError::Connection => Self::Repo,
        }
    }
}

impl<'r, R> FindById<'r, R>
where
    R: Repo,
{
    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("Find thought by ID: {:?}", req);
        let thought_record = self.repo.get(req.id)?;
        Ok(Response::from(thought_record))
    }
}
