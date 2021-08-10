use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    Id,
    NotFound,
}

#[cfg(feature = "conversions")]
mod conv {
    use super::*;
    use crate::domain::Thought;
    use application::usecase::thought::find_by_id as uc;
    use std::convert::TryFrom;

    impl<Id> From<uc::Response<Id>> for Thought
    where
        Id: ToString,
    {
        fn from(from: uc::Response<Id>) -> Self {
            let uc::Response { id, title } = from;
            let id = id.to_string();
            Self { id, title }
        }
    }

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
