use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {}

#[cfg(feature = "conversions")]
mod conv {
    use super::*;
    use crate::domain::AreaOfLife;
    use application::usecase::area_of_life::read_all as uc;
    use std::convert::TryFrom;

    impl From<uc::AreaOfLife> for AreaOfLife {
        fn from(from: uc::AreaOfLife) -> Self {
            let uc::AreaOfLife { id, name } = from;
            let id = id.to_u64().into();
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
