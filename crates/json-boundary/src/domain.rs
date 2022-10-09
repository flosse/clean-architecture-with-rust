use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thought {
    pub id: ThoughtId,
    pub title: String,
    pub areas_of_life: Vec<AreaOfLifeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaOfLife {
    pub id: AreaOfLifeId,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThoughtId(pub u64);

impl From<u64> for ThoughtId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl fmt::Display for ThoughtId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AreaOfLifeId(pub u64);

impl From<u64> for AreaOfLifeId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl fmt::Display for AreaOfLifeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
