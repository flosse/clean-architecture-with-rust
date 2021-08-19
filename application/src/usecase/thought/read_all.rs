use crate::gateway::repository::thought::{GetAllError, Repo, ThoughtRecord};
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug)]
pub struct Request;

#[derive(Debug)]
pub struct Response<Id> {
    pub thoughts: Vec<Thought<Id>>,
}

#[derive(Debug)]
pub struct Thought<Id> {
    pub id: Id,
    pub title: String,
}

impl<Id> From<ThoughtRecord<Id>> for Thought<Id> {
    fn from(r: ThoughtRecord<Id>) -> Self {
        let ThoughtRecord { id, thought } = r;
        let title = thought.title.into_string();
        Self { id, title }
    }
}

type Id<R> = <R as Repo>::Id;

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
    Id<R>: Clone + Debug,
{
    pub fn exec(&self, _: Request) -> Result<Response<Id<R>>, Error> {
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
