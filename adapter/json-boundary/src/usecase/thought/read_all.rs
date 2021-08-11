use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {}

#[cfg(feature = "conversions")]
mod conv {
    use super::*;
    use crate::domain::Thought;
    use application::usecase::thought::read_all as uc;
    use std::convert::TryFrom;

    impl<Id> From<uc::Thought<Id>> for Thought
    where
        Id: ToString,
    {
        fn from(from: uc::Thought<Id>) -> Self {
            let uc::Thought { id, title } = from;
            let id = id.to_string();
            Self { id, title }
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
