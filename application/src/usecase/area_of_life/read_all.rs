use crate::gateway::repository::area_of_life::{GetAllError, Record, Repo};
use domain::area_of_life::Id;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug)]
pub struct Request;

#[derive(Debug)]
pub struct Response {
    pub areas_of_life: Vec<AreaOfLife>,
}

#[derive(Debug)]
pub struct AreaOfLife {
    pub id: Id,
    pub name: String,
}

impl From<Record> for AreaOfLife {
    fn from(r: Record) -> Self {
        let Record { area_of_life } = r;
        let name = String::from(area_of_life.name().as_ref());
        let id = area_of_life.id();
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

impl<'r, R> ReadAll<'r, R>
where
    R: Repo,
{
    pub fn exec(&self, _: Request) -> Result<Response, Error> {
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
