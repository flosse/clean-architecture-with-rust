use domain::AreaOfLife;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetError {
    #[error("Area of life not found")]
    NotFound,
    #[error("Area of life repository connection problem")]
    Connection,
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("Area of life repository connection problem")]
    Connection,
}

#[derive(Debug, Error)]
pub enum GetAllError {
    #[error("Area of life repository connection problem")]
    Connection,
}

#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("Area of life not found")]
    NotFound,
    #[error("Area of life repository connection problem")]
    Connection,
}

#[derive(Debug, Clone)]
pub struct AreaOfLifeRecord<Id> {
    pub id: Id,
    pub area_of_life: AreaOfLife,
}

pub trait Repo: Send + Sync {
    type Id;
    fn save(&self, record: AreaOfLifeRecord<Self::Id>) -> Result<(), SaveError>;
    fn get(&self, id: Self::Id) -> Result<AreaOfLifeRecord<Self::Id>, GetError>;
    fn get_all(&self) -> Result<Vec<AreaOfLifeRecord<Self::Id>>, GetAllError>;
    fn delete(&self, id: Self::Id) -> Result<(), DeleteError>;
}
