use entity::item::Item;
use std::{io, result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Item not found")]
    NotFound,
    #[error(transparent)]
    Io(#[from] io::Error),
}

pub type Result<T> = result::Result<T, Error>;

pub trait ItemRepo: Send + Sync {
    type Id;
    fn save(&self, item: Item) -> Result<Self::Id>;
    fn get(&self, id: Self::Id) -> Result<Item>;
}
