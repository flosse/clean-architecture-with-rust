use crate::domain::{AreaOfLifeId, ThoughtId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub id: ThoughtId,
    pub title: String,
    pub areas_of_life: Vec<AreaOfLifeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    Id,
    NotFound(ThoughtId),
    AreaOfLifeId,
    TitleMinLength { min: usize, actual: usize },
    TitleMaxLength { max: usize, actual: usize },
    AreasOfLifeNotFound(Vec<AreaOfLifeId>),
}

#[cfg(feature = "conversions")]
mod conv {
    use super::*;
    use application::usecase::thought::validate;

    impl From<validate::ThoughtInvalidity> for Error {
        fn from(from: validate::ThoughtInvalidity) -> Self {
            let validate::ThoughtInvalidity::Title(e) = from;
            use validate::TitleInvalidity as T;
            match e {
                T::MinLength { min, actual } => Self::TitleMinLength { min, actual },
                T::MaxLength { max, actual } => Self::TitleMaxLength { max, actual },
            }
        }
    }
}
