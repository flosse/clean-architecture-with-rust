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

pub trait Repo: Send + Sync {
    type Id;
    fn save(&self, thought: Thought) -> Result<Self::Id, SaveError>;
    fn get(&self, id: Self::Id) -> Result<Thought, GetError>;
    fn get_all(&self) -> Result<Vec<(Self::Id, Thought)>, GetAllError>;
}
