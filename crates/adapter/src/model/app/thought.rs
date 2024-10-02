use std::{fmt, str::FromStr};

use thiserror::Error;

use cawr_domain::thought;

/// This is the public ID of a thought.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(u64);

impl Id {
    #[must_use]
    pub const fn to_u64(self) -> u64 {
        self.0
    }
}

impl From<thought::Id> for Id {
    fn from(id: thought::Id) -> Self {
        Self(id.to_u64())
    }
}

impl From<Id> for thought::Id {
    fn from(id: Id) -> Self {
        Self::new(id.0)
    }
}

#[derive(Debug, Error)]
#[error("Unable to parse thought ID")]
pub struct ParseIdError;

impl FromStr for Id {
    type Err = ParseIdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s.parse().map_err(|_| ParseIdError)?;
        Ok(Self(id))
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub mod create {
    use crate::model::app::area_of_life as aol;
    use cawr_application::usecase::thought::{create as uc, validate::ThoughtInvalidity};
    use std::{collections::HashSet, result};
    use thiserror::Error;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("{}", aol::ParseIdError)]
        AreaOfLifeId,
        #[error("{}", uc::Error::NewId)]
        NewId,
        #[error("{}", uc::Error::Repo)]
        Repo,
        #[error(transparent)]
        Invalidity(#[from] ThoughtInvalidity),
        #[error("Areas of life {0:?} not found")]
        AreasOfLifeNotFound(HashSet<aol::Id>),
    }

    impl From<aol::ParseIdError> for Error {
        fn from(_: aol::ParseIdError) -> Self {
            Self::AreaOfLifeId
        }
    }
    impl From<uc::Error> for Error {
        fn from(from: uc::Error) -> Self {
            match from {
                uc::Error::NewId => Self::NewId,
                uc::Error::Repo => Self::Repo,
                uc::Error::Invalidity(i) => Self::Invalidity(i),
                uc::Error::AreasOfLifeNotFound(ids) => {
                    Self::AreasOfLifeNotFound(ids.into_iter().map(Into::into).collect())
                }
            }
        }
    }
}

pub mod update {
    use super::ParseIdError;
    use crate::model::app::{area_of_life as aol, thought::Id};
    use cawr_application::usecase::thought::{update as uc, validate::ThoughtInvalidity};
    use std::{collections::HashSet, result};
    use thiserror::Error;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("{}", ParseIdError)]
        Id,
        #[error("Thought {0:?} not found")]
        NotFound(Id),
        #[error("{}", aol::ParseIdError)]
        AreaOfLifeId,
        #[error("{}", uc::Error::Repo)]
        Repo,
        #[error(transparent)]
        Invalidity(#[from] ThoughtInvalidity),
        #[error("Areas of life {0:?} not found")]
        AreasOfLifeNotFound(HashSet<aol::Id>),
    }

    impl From<aol::ParseIdError> for Error {
        fn from(_: aol::ParseIdError) -> Self {
            Self::AreaOfLifeId
        }
    }
    impl From<uc::Error> for Error {
        fn from(from: uc::Error) -> Self {
            match from {
                uc::Error::Repo => Self::Repo,
                uc::Error::Invalidity(i) => Self::Invalidity(i),
                uc::Error::ThoughtNotFound(id) => Self::NotFound(Id::from(id)),
                uc::Error::AreasOfLifeNotFound(ids) => {
                    Self::AreasOfLifeNotFound(ids.into_iter().map(Into::into).collect())
                }
            }
        }
    }
}

pub mod find_by_id {
    use super::ParseIdError;
    use cawr_application::usecase::thought::find_by_id as uc;
    use std::result;
    use thiserror::Error;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("{}", ParseIdError)]
        Id,
        #[error("{}", uc::Error::NotFound)]
        NotFound,
        #[error("{}", uc::Error::Repo)]
        Repo,
    }

    impl From<uc::Error> for Error {
        fn from(e: uc::Error) -> Self {
            match e {
                uc::Error::Repo => Error::Repo,
                uc::Error::NotFound => Error::NotFound,
            }
        }
    }
}

pub mod read_all {
    use cawr_application::usecase::thought::read_all as uc;
    use std::result;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;
    pub type Error = uc::Error;
}

pub mod delete {
    use super::ParseIdError;
    use cawr_application::usecase::thought::delete as uc;
    use std::result;
    use thiserror::Error;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("{}", ParseIdError)]
        Id,
        #[error("{}", uc::Error::NotFound)]
        NotFound,
        #[error("{}", uc::Error::Repo)]
        Repo,
    }

    impl From<uc::Error> for Error {
        fn from(e: uc::Error) -> Self {
            match e {
                uc::Error::Repo => Error::Repo,
                uc::Error::NotFound => Error::NotFound,
            }
        }
    }
}
