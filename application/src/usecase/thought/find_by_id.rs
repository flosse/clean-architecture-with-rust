use crate::gateway::repository::thought::{Error as RepoError, Repo};
use thiserror::Error;

pub struct Request<Id> {
    pub id: Id,
}

pub struct Response<Id> {
    pub id: Id,
    pub title: String,
}

type Id<R> = <R as Repo>::Id;

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
    #[error(transparent)]
    Repo(#[from] RepoError),
}

impl<'r, R> FindById<'r, R>
where
    R: Repo,
    Id<R>: Clone,
{
    pub fn exec(&self, req: Request<Id<R>>) -> Result<Response<Id<R>>, Error> {
        let thought = self.repo.get(req.id.clone())?;
        Ok(Response {
            id: req.id,
            title: thought.title.into_string(),
        })
    }
}