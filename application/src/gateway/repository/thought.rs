use domain::thought::Thought;
use std::{io, result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Though not found")]
    NotFound,
    #[error(transparent)]
    Io(#[from] io::Error),
}

pub type Result<T> = result::Result<T, Error>;

pub trait Repo: Send + Sync {
    type Id;
    fn save(&self, thought: Thought) -> Result<Self::Id>;
    fn get(&self, id: Self::Id) -> Result<Thought>;
}
