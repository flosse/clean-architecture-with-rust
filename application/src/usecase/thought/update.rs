use crate::{
    gateway::repository::{
        area_of_life,
        thought::{self, GetError, Record, SaveError},
    },
    usecase::{
        area_of_life::check_existence::{self as check_aol, CheckAreasOfLifeExistence},
        thought::validate::{self, validate_thought_properties, ThoughtInvalidity},
    },
};
use domain::{
    area_of_life as aol,
    thought::{Id, Thought, Title},
};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    /// The id of the thought.
    pub id: Id,
    /// The title of the thought.
    pub title: String,
    /// Associated [aol::AreaOfLife]s.
    pub areas_of_life: HashSet<aol::Id>,
}

pub type Response = ();

/// Update thought usecase interactor
pub struct UpdateThought<'r, R> {
    repo: &'r R,
}

impl<'r, R> UpdateThought<'r, R> {
    pub const fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("Thought {0} not found")]
    ThoughtNotFound(Id),
    #[error(transparent)]
    Invalidity(#[from] ThoughtInvalidity),
    #[error("Areas of life {0:?} not found")]
    AreasOfLifeNotFound(HashSet<aol::Id>),
}

impl From<SaveError> for Error {
    fn from(err: SaveError) -> Self {
        match err {
            SaveError::Connection => Self::Repo,
        }
    }
}

impl From<(Id, GetError)> for Error {
    fn from((id, err): (Id, GetError)) -> Self {
        match err {
            GetError::Connection => Error::Repo,
            GetError::NotFound => Error::ThoughtNotFound(id),
        }
    }
}

impl From<check_aol::Error> for Error {
    fn from(e: check_aol::Error) -> Self {
        use check_aol::Error as E;
        match e {
            E::Repo => Error::Repo,
            E::NotFound(aol_ids) => Error::AreasOfLifeNotFound(aol_ids),
        }
    }
}

impl<'r, R> UpdateThought<'r, R>
where
    R: thought::Repo + area_of_life::Repo,
{
    /// Update a thought.
    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("Update thought: {:?}", req);
        validate_thought_properties(&validate::Request { title: &req.title })?;
        CheckAreasOfLifeExistence::new(self.repo).exec(&req.areas_of_life)?;
        thought::Repo::get(self.repo, req.id).map_err(|err| (req.id, err))?;
        let title = Title::new(req.title);
        let thought = Thought::new(req.id, title, req.areas_of_life);
        let record = Record { thought };
        thought::Repo::save(self.repo, record)?;
        Ok(())
    }
}
