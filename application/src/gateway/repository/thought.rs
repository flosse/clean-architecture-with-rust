use domain::thought::{Id, Thought};
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
pub struct Record {
    pub thought: Thought,
}

// TODO: make it async
pub trait Repo: Send + Sync {
    fn save(&self, record: Record) -> Result<(), SaveError>;
    fn get(&self, id: Id) -> Result<Record, GetError>;
    fn get_all(&self) -> Result<Vec<Record>, GetAllError>;
    fn delete(&self, id: Id) -> Result<(), DeleteError>;
}
