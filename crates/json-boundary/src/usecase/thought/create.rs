use crate::domain::AreaOfLifeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub title: String,
    pub areas_of_life: Vec<AreaOfLifeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    AreaOfLifeId,
    TitleMinLength { min: usize, actual: usize },
    TitleMaxLength { max: usize, actual: usize },
    AreasOfLifeNotFound(Vec<AreaOfLifeId>),
}
