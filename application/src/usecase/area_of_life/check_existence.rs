use crate::gateway::repository::area_of_life::{GetError, Repo};
use cawr_domain::area_of_life::Id;
use std::collections::HashSet;
use thiserror::Error;

pub type Request<'a> = &'a HashSet<Id>;

pub struct CheckAreasOfLifeExistence<'r, R> {
    repo: &'r R,
}

impl<'r, R> CheckAreasOfLifeExistence<'r, R> {
    pub const fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", GetError::Connection)]
    Repo,
    #[error("Area of life {0:?} not found")]
    NotFound(HashSet<Id>),
}

impl<'r, R> CheckAreasOfLifeExistence<'r, R>
where
    R: Repo,
{
    pub fn exec(&self, req: Request) -> Result<(), Error> {
        let mut not_found = HashSet::new();
        for id in req {
            match self.repo.get(*id) {
                Err(GetError::Connection) => {
                    return Err(Error::Repo);
                }
                Err(GetError::NotFound) => {
                    not_found.insert(*id);
                }
                Ok(_) => {}
            }
        }
        if not_found.is_empty() {
            Ok(())
        } else {
            Err(Error::NotFound(not_found))
        }
    }
}
