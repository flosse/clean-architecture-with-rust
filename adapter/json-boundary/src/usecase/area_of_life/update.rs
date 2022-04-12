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

#[cfg(feature = "conversions")]
mod conv {
    use super::*;
    use application::usecase::area_of_life::validate;

    impl From<validate::AreaOfLifeInvalidity> for Error {
        fn from(from: validate::AreaOfLifeInvalidity) -> Self {
            let validate::AreaOfLifeInvalidity::Name(e) = from;
            use validate::NameInvalidity as T;
            match e {
                T::MinLength { min, actual } => Self::NameMinLength { min, actual },
                T::MaxLength { max, actual } => Self::NameMaxLength { max, actual },
            }
        }
    }
}
