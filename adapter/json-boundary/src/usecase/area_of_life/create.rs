use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    NameMinLength { min: usize, actual: usize },
    NameMaxLength { max: usize, actual: usize },
}

#[cfg(feature = "conversions")]
mod conv {
    use super::*;
    use crate::domain::AreaOfLifeId;
    use application::usecase::area_of_life::{create as uc, validate};
    use std::convert::TryFrom;

    impl<Id> From<uc::Response<Id>> for AreaOfLifeId
    where
        Id: ToString,
    {
        fn from(from: uc::Response<Id>) -> Self {
            AreaOfLifeId(from.id.to_string())
        }
    }

    impl TryFrom<uc::Error> for Error {
        type Error = ();
        fn try_from(from: uc::Error) -> Result<Self, Self::Error> {
            use uc::Error as E;
            match from {
                E::Repo | E::NewId => Err(()),
                E::Invalidity(e) => {
                    let validate::AreaOfLifeInvalidity::Name(e) = e;
                    use validate::NameInvalidity as T;
                    Ok(match e {
                        T::MinLength { min, actual } => Self::NameMinLength { min, actual },
                        T::MaxLength { max, actual } => Self::NameMaxLength { max, actual },
                    })
                }
            }
        }
    }
}
