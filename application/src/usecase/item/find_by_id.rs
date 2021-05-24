use crate::gateway::repository::item::{Error as RepoError, ItemRepo};
use thiserror::Error;

pub struct Request<Id> {
    pub id: Id,
}

pub struct Response<Id> {
    pub id: Id,
    pub title: String,
}

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

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Repo(#[from] RepoError),
}

impl<'r, R> FindById<'r, R>
where
    R: ItemRepo,
    Id<R>: Clone,
{
    pub fn exec(&self, req: Request<Id<R>>) -> Result<Response<Id<R>>, Error> {
        let item = self.repo.get(req.id.clone())?;
        Ok(Response {
            id: req.id,
            title: item.title.into_string(),
        })
    }
}
