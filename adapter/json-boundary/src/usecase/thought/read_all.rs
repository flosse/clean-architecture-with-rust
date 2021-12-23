use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {}

#[cfg(feature = "conversions")]
mod conv {
    use super::*;
    use crate::domain::Thought;
    use application::usecase::thought::read_all as uc;
    use std::convert::TryFrom;

    impl From<uc::Thought> for Thought {
        fn from(from: uc::Thought) -> Self {
            let uc::Thought {
                id,
                title,
                areas_of_life,
            } = from;
            let id = id.to_u64().into();
            let areas_of_life = areas_of_life
                .into_iter()
                .map(|id| id.to_u64().into())
                .collect();
            Self {
                id,
                title,
                areas_of_life,
            }
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
