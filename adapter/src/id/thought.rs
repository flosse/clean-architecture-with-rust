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
pub struct ParseError;

impl FromStr for Id {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s.parse().map_err(|_| ParseError)?;
        Ok(Self(id))
    }
}

impl ToString for Id {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
