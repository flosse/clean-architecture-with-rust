use crate::gateway::repository::item::ItemRepo;
use std::{error, fmt};
use thiserror::Error;

pub struct Request<Id> {
    pub id: Id,
}

pub struct Response<Id> {
    pub id: Id,
    pub title: String,
}

type RepoError<R> = <R as ItemRepo>::Err;
type Id<R> = <R as ItemRepo>::Id;

/// Find item by ID usecase interactor
pub struct FindById<'r, R> {
    repo: &'r R,
}

impl<'r, R> FindById<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Error)]
pub enum Error<R>
where
    R: ItemRepo,
    RepoError<R>: error::Error + 'static,
{
    #[error(transparent)]
    Repo(RepoError<R>),
}

impl<R> fmt::Debug for Error<R>
where
    R: ItemRepo,
    RepoError<R>: error::Error,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Repo(e) => e.fmt(f),
        }
    }
}

impl<'r, R> FindById<'r, R>
where
    R: ItemRepo,
    RepoError<R>: error::Error,
    Id<R>: Clone,
{
    pub fn exec(&self, req: Request<Id<R>>) -> Result<Response<Id<R>>, Error<R>> {
        let item = self.repo.get(req.id.clone()).map_err(Error::Repo)?;
        Ok(Response {
            id: req.id,
            title: item.title.into_string(),
        })
    }
}
