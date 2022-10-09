use cawr_domain::area_of_life::{AreaOfLife, Id};
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
pub struct Record {
    pub area_of_life: AreaOfLife,
}

// TODO: make it async
pub trait Repo: Send + Sync {
    fn save(&self, record: Record) -> Result<(), SaveError>;
    fn get(&self, id: Id) -> Result<Record, GetError>;
    fn get_all(&self) -> Result<Vec<Record>, GetAllError>;
    fn delete(&self, id: Id) -> Result<(), DeleteError>;
}
