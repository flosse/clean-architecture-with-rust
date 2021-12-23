use crate::gateway::repository::thought::{GetAllError, Record, Repo};
use domain::{area_of_life as aol, thought::Id};
use std::{collections::HashSet, fmt::Debug};
use thiserror::Error;

#[derive(Debug)]
pub struct Request;

#[derive(Debug)]
pub struct Response {
    pub thoughts: Vec<Thought>,
}

#[derive(Debug)]
pub struct Thought {
    pub id: Id,
    pub title: String,
    pub areas_of_life: HashSet<aol::Id>,
}

impl From<Record> for Thought {
    fn from(r: Record) -> Self {
        let Record { thought } = r;
        let title = String::from(thought.title().as_ref());
        let id = thought.id();
        let areas_of_life = thought.areas_of_life().clone();
        Self {
            id,
            title,
            areas_of_life,
        }
    }
}

/// Read all thoughts usecase interactor
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
        log::debug!("Read all thoughts");
        let thoughts = self
            .repo
            .get_all()?
            .into_iter()
            .map(Thought::from)
            .collect();
        Ok(Response { thoughts })
    }
}
