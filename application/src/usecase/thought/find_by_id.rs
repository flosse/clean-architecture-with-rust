use crate::gateway::repository::thought::{GetError, Repo, ThoughtRecord};
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug)]
pub struct Request<Id> {
    pub id: Id,
}

#[derive(Debug)]
pub struct Response<Id> {
    pub id: Id,
    pub title: String,
}

impl<Id> From<ThoughtRecord<Id>> for Response<Id> {
    fn from(r: ThoughtRecord<Id>) -> Self {
        let ThoughtRecord { id, thought } = r;
        let title = thought.title.into_string();
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

impl<'r, Id, R> FindById<'r, R>
where
    R: Repo<Id = Id>,
    Id: Clone + Debug,
{
    pub fn exec(&self, req: Request<Id>) -> Result<Response<Id>, Error> {
        log::debug!("Find thought by ID: {:?}", req);
        let thought_record = self.repo.get(req.id)?;
        Ok(Response::from(thought_record))
    }
}
