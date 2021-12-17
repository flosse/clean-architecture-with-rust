use domain::thought;
use std::str::FromStr;
use thiserror::Error;

/// This is the public ID of a thought.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(u64);

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

impl ToString for Id {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub mod create {
    use application::usecase::thought::create as uc;
    use std::result;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;
    pub type Error = uc::Error;
}

pub mod find_by_id {
    use super::ParseIdError;
    use application::usecase::thought::find_by_id as uc;
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

pub mod read_all {
    use application::usecase::thought::read_all as uc;
    use std::result;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;
    pub type Error = uc::Error;
}

pub mod delete {
    use super::ParseIdError;
    use application::usecase::thought::delete as uc;
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
