use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    TitleMinLength { min: usize, actual: usize },
    TitleMaxLength { max: usize, actual: usize },
}

#[cfg(feature = "conversions")]
mod conv {
    use super::*;
    use crate::domain::ThoughtId;
    use application::usecase::thought::{create as uc, validate};
    use std::convert::TryFrom;

    impl From<uc::Response> for ThoughtId {
        fn from(from: uc::Response) -> Self {
            from.id.to_u64().into()
        }
    }

    impl TryFrom<uc::Error> for Error {
        type Error = ();
        fn try_from(from: uc::Error) -> Result<Self, Self::Error> {
            use uc::Error as E;
            match from {
                E::Repo | E::NewId => Err(()),
                E::Invalidity(e) => {
                    let validate::ThoughtInvalidity::Title(e) = e;
                    use validate::TitleInvalidity as T;
                    Ok(match e {
                        T::MinLength { min, actual } => Self::TitleMinLength { min, actual },
                        T::MaxLength { max, actual } => Self::TitleMaxLength { max, actual },
                    })
                }
            }
        }
    }
}
