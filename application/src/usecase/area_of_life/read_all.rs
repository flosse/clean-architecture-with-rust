use crate::gateway::repository::area_of_life::{AreaOfLifeRecord, GetAllError, Repo};
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug)]
pub struct Request;

#[derive(Debug)]
pub struct Response<Id> {
    pub areas_of_life: Vec<AreaOfLife<Id>>,
}

#[derive(Debug)]
pub struct AreaOfLife<Id> {
    pub id: Id,
    pub name: String,
}

impl<Id> From<AreaOfLifeRecord<Id>> for AreaOfLife<Id> {
    fn from(r: AreaOfLifeRecord<Id>) -> Self {
        let AreaOfLifeRecord { id, area_of_life } = r;
        let name = String::from(area_of_life.name);
        Self { id, name }
    }
}

/// Read all areas of life usecase interactor
pub struct ReadAll<'r, R> {
    repo: &'r R,
}

impl<'r, R> ReadAll<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", GetAllError::Connection)]
    Repo,
}

impl From<GetAllError> for Error {
    fn from(e: GetAllError) -> Self {
        match e {
            GetAllError::Connection => Self::Repo,
        }
    }
}

impl<'r, Id, R> ReadAll<'r, R>
where
    R: Repo<Id = Id>,
    Id: Clone + Debug,
{
    pub fn exec(&self, _: Request) -> Result<Response<Id>, Error> {
        log::debug!("Read all areas of life");
        let areas_of_life = self
            .repo
            .get_all()?
            .into_iter()
            .map(AreaOfLife::from)
            .collect();
        Ok(Response { areas_of_life })
    }
}
