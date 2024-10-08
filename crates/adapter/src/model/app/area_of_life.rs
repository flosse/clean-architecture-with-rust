use std::{fmt, str::FromStr};

use thiserror::Error;

use cawr_domain::area_of_life as aol;

/// This is the public ID of an area of life.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(u64);

impl Id {
    #[must_use]
    pub const fn to_u64(self) -> u64 {
        self.0
    }
}

impl From<aol::Id> for Id {
    fn from(id: aol::Id) -> Self {
        Self(id.to_u64())
    }
}

impl From<Id> for aol::Id {
    fn from(id: Id) -> Self {
        Self::new(id.0)
    }
}

#[derive(Debug, Error)]
#[error("Unable to parse area of life ID")]
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
    use cawr_application::usecase::area_of_life::create as uc;
    use std::result;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;
    pub type Error = uc::Error;
}

pub mod update {
    use super::{Id, ParseIdError};
    use cawr_application::usecase::area_of_life::{update as uc, validate::AreaOfLifeInvalidity};
    use std::result;
    use thiserror::Error;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("{}", ParseIdError)]
        Id,
        #[error("Area of life {0:?} not found")]
        NotFound(Id),
        #[error("{}", uc::Error::Repo)]
        Repo,
        #[error(transparent)]
        Invalidity(#[from] AreaOfLifeInvalidity),
    }

    impl From<ParseIdError> for Error {
        fn from(_: ParseIdError) -> Self {
            Self::Id
        }
    }

    impl From<uc::Error> for Error {
        fn from(from: uc::Error) -> Self {
            match from {
                uc::Error::NotFound(id) => Self::NotFound(id.into()),
                uc::Error::Invalidity(i) => Self::Invalidity(i),
                uc::Error::Repo => Self::Repo,
            }
        }
    }
}

pub mod read_all {
    use cawr_application::usecase::area_of_life::read_all as uc;
    use std::result;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;
    pub type Error = uc::Error;
}

pub mod delete {
    use super::ParseIdError;
    use cawr_application::usecase::area_of_life::delete as uc;
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
}
