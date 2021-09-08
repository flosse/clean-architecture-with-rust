use crate::{
    gateway::repository::area_of_life::{AreaOfLifeRecord, Repo, SaveError},
    identifier::{NewId, NewIdError},
    usecase::area_of_life::validate::{validate_area_of_life, AreaOfLifeInvalidity},
};
use domain::area_of_life::{AreaOfLife, Name};
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    /// The title of the new area of life.
    pub name: String,
}

#[derive(Debug)]
pub struct Response<Id> {
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

impl<'r, 'g, Id, R, G> CreateAreaOfLife<'r, 'g, R, G>
where
    R: Repo<Id = Id>,
    G: NewId<Id>,
    Id: Clone + Copy,
{
    /// Create a new area of life with the given name.
    pub fn exec(&self, req: Request) -> Result<Response<Id>, Error> {
        log::debug!("Create new area of life: {:?}", req);
        let name = Name::new(req.name);
        let area_of_life = AreaOfLife { name };
        validate_area_of_life(&area_of_life)?;
        let id = self.id_gen.new_id().map_err(|err| {
            log::warn!("{}", err);
            Error::NewId
        })?;
        let record = AreaOfLifeRecord { id, area_of_life };
        self.repo.save(record)?;
        Ok(Response { id })
    }
}
