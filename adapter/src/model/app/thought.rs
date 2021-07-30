use std::str::FromStr;
use thiserror::Error;

/// This is the public ID of a thought.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(u32);

impl From<u32> for Id {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl From<Id> for u32 {
    fn from(id: Id) -> Self {
        id.0
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
    use super::Id;
    use application::usecase::thought::create as uc;
    use std::result;

    pub type Request = uc::Request;
    pub type Response = uc::Response<Id>;
    pub type Result = result::Result<Response, Error>;
    pub type Error = uc::Error;
}

pub mod find_by_id {
    use super::{Id, ParseIdError};
    use application::{gateway::repository::thought as repo, usecase::thought::find_by_id as uc};
    use std::result;
    use thiserror::Error;

    pub type Request = uc::Request<Id>;
    pub type Response = uc::Response<Id>;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error(transparent)]
        Id(#[from] ParseIdError),
        #[error(transparent)]
        Repo(#[from] repo::Error),
    }
}
