use std::str::FromStr;
use thiserror::Error;

/// This is the public ID of an item.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(u32);

impl From<u32> for ItemId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl From<ItemId> for u32 {
    fn from(id: ItemId) -> Self {
        id.0
    }
}

#[derive(Debug, Error)]
#[error("Unable to parse item ID")]
pub struct ParseItemIdError;

impl FromStr for ItemId {
    type Err = ParseItemIdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s.parse().map_err(|_| ParseItemIdError)?;
        Ok(Self(id))
    }
}

impl ToString for ItemId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
