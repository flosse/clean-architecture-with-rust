use domain::thought::Thought;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetError {
    #[error("Thought not found")]
    NotFound,
    #[error("Thought repository connection problem")]
    Connection,
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("Thought repository connection problem")]
    Connection,
}

#[derive(Debug, Error)]
pub enum GetAllError {
    #[error("Thought repository connection problem")]
    Connection,
}

#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("Thought not found")]
    NotFound,
    #[error("Thought repository connection problem")]
    Connection,
}

#[derive(Debug, Clone)]
pub struct ThoughtRecord<Id> {
    pub id: Id,
    pub thought: Thought,
}

pub trait Repo: Send + Sync {
    type Id;
    fn save(&self, record: ThoughtRecord<Self::Id>) -> Result<(), SaveError>;
    fn get(&self, id: Self::Id) -> Result<ThoughtRecord<Self::Id>, GetError>;
    fn get_all(&self) -> Result<Vec<ThoughtRecord<Self::Id>>, GetAllError>;
    fn delete(&self, id: Self::Id) -> Result<(), DeleteError>;
}

/// A service that generates a new thought ID.
// The creation of the ID should be done **before** we save a record.
// To do that we delegate the generation of a new ID to a separate
// service that can be injected e.g. into a specific usecase.
// See: https://matthiasnoback.nl/2018/05/when-and-where-to-determine-the-id-of-an-entity/
pub trait NewId<Id> {
    fn new_id(&self) -> Result<Id, NewIdError>;
}

#[derive(Debug, Error)]
#[error("Unable to generade a new thought ID")]
pub struct NewIdError;
