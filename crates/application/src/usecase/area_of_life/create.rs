use crate::{
    gateway::repository::area_of_life::{Record, Repo, SaveError},
    identifier::{NewId, NewIdError},
    usecase::area_of_life::validate::{
        self, validate_area_of_life_properties, AreaOfLifeInvalidity,
    },
};
use cawr_domain::area_of_life::{AreaOfLife, Id, Name};
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    /// The title of the new area of life.
    pub name: String,
}

#[derive(Debug)]
pub struct Response {
    /// The ID of the newly created area of life.
    pub id: Id,
}

/// Create area of life usecase interactor
pub struct CreateAreaOfLife<'r, 'g, R, G> {
    repo: &'r R,
    id_gen: &'g G,
}

impl<'r, 'g, R, G> CreateAreaOfLife<'r, 'g, R, G> {
    pub fn new(repo: &'r R, id_gen: &'g G) -> Self {
        Self { repo, id_gen }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("{}", NewIdError)]
    NewId,
    #[error(transparent)]
    Invalidity(#[from] AreaOfLifeInvalidity),
}

impl From<SaveError> for Error {
    fn from(e: SaveError) -> Self {
        match e {
            SaveError::Connection => Self::Repo,
        }
    }
}

impl<'r, 'g, R, G> CreateAreaOfLife<'r, 'g, R, G>
where
    R: Repo,
    G: NewId<Id>,
{
    /// Create a new area of life with the given name.
    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("Create new area of life: {:?}", req);
        validate_area_of_life_properties(&validate::Request { name: &req.name })?;
        let name = Name::new(req.name);
        let id = self.id_gen.new_id().map_err(|err| {
            log::warn!("{}", err);
            Error::NewId
        })?;
        let area_of_life = AreaOfLife::new(id, name);
        let record = Record { area_of_life };
        self.repo.save(record)?;
        Ok(Response { id })
    }
}
