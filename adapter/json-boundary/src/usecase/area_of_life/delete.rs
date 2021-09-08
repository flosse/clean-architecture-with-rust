use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    Id,
    NotFound,
}

#[cfg(feature = "conversions")]
mod conv {
    use super::*;
    use application::usecase::area_of_life::delete as uc;
    use std::convert::TryFrom;

    impl TryFrom<uc::Error> for Error {
        type Error = ();
        fn try_from(from: uc::Error) -> Result<Self, Self::Error> {
            match from {
                uc::Error::NotFound => Ok(Self::NotFound),
                uc::Error::Repo => Err(()),
            }
        }
    }
}
