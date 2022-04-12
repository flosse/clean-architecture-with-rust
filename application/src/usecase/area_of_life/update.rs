use crate::{
    gateway::repository::area_of_life::{GetError, Record, Repo, SaveError},
    usecase::area_of_life::validate::{
        self, validate_area_of_life_properties, AreaOfLifeInvalidity,
    },
};
use domain::area_of_life::{AreaOfLife, Id, Name};
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    /// The id of the area of life.
    pub id: Id,
    /// The name of the area of life.
    pub name: String,
}

pub type Response = ();

/// Update area of life usecase interactor
pub struct UpdateAreaOfLife<'r, R> {
    repo: &'r R,
}

impl<'r, R> UpdateAreaOfLife<'r, R> {
    pub const fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Area of life {0} not found")]
    NotFound(Id),
    #[error(transparent)]
    Invalidity(#[from] AreaOfLifeInvalidity),
    #[error("{}", SaveError::Connection)]
    Repo,
}

impl From<SaveError> for Error {
    fn from(err: SaveError) -> Self {
        match err {
            SaveError::Connection => Self::Repo,
        }
    }
}

impl From<(GetError, Id)> for Error {
    fn from((err, id): (GetError, Id)) -> Self {
        match err {
            GetError::NotFound => Self::NotFound(id),
            GetError::Connection => Self::Repo,
        }
    }
}

impl<'r, R> UpdateAreaOfLife<'r, R>
where
    R: Repo,
{
    /// Update a area of life.
    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("Update area of life: {:?}", req);
        validate_area_of_life_properties(&validate::Request { name: &req.name })?;
        let name = Name::new(req.name);
        let area_of_life = AreaOfLife { id: req.id, name };
        let record = Record { area_of_life };
        let _ = self.repo.get(req.id).map_err(|err| (err, req.id))?;
        self.repo.save(record)?;
        Ok(())
    }
}
