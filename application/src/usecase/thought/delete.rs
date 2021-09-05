use crate::gateway::repository::thought::{DeleteError, Repo};
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug)]
pub struct Request<Id> {
    pub id: Id,
}

#[derive(Debug)]
pub struct Response;

/// Delete thought by ID usecase interactor
pub struct Delete<'r, R> {
    repo: &'r R,
}

impl<'r, R> Delete<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", DeleteError::NotFound)]
    NotFound,
    #[error("{}", DeleteError::Connection)]
    Repo,
}

impl From<DeleteError> for Error {
    fn from(e: DeleteError) -> Self {
        match e {
            DeleteError::NotFound => Self::NotFound,
            DeleteError::Connection => Self::Repo,
        }
    }
}

impl<'r, Id, R> Delete<'r, R>
where
    R: Repo<Id = Id>,
    Id: Clone + Debug,
{
    pub fn exec(&self, req: Request<Id>) -> Result<Response, Error> {
        log::debug!("Delete thought by ID: {:?}", req);
        self.repo.delete(req.id)?;
        Ok(Response {})
    }
}
