use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {}

#[cfg(feature = "conversions")]
mod conv {
    use super::*;
    use crate::domain::AreaOfLife;
    use application::usecase::area_of_life::read_all as uc;
    use std::convert::TryFrom;

    impl<Id> From<uc::AreaOfLife<Id>> for AreaOfLife
    where
        Id: ToString,
    {
        fn from(from: uc::AreaOfLife<Id>) -> Self {
            let uc::AreaOfLife { id, name } = from;
            let id = id.to_string().into();
            Self { id, name }
        }
    }

    impl TryFrom<uc::Error> for Error {
        type Error = ();
        fn try_from(from: uc::Error) -> Result<Self, Self::Error> {
            match from {
                uc::Error::Repo => Err(()),
            }
        }
    }
}
