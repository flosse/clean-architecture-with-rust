use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Thought {
    pub id: ThoughtId,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaOfLife {
    pub id: AreaOfLifeId,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThoughtId(pub u64);

impl From<u64> for ThoughtId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AreaOfLifeId(pub u64);

impl From<u64> for AreaOfLifeId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}
