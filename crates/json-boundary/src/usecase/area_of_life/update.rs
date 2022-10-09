use crate::domain::AreaOfLifeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub id: AreaOfLifeId,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    Id,
    NotFound,
    NameMinLength { min: usize, actual: usize },
    NameMaxLength { max: usize, actual: usize },
}
