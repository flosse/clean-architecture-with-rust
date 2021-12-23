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
}

#[cfg(feature = "conversions")]
mod conv {
    use super::*;
    use crate::domain::ThoughtId;
    use application::usecase::thought::{create as uc, validate};

    impl From<uc::Response> for ThoughtId {
        fn from(from: uc::Response) -> Self {
            from.id.to_u64().into()
        }
    }

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
