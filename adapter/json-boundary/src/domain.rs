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
pub struct ThoughtId(pub String);

impl From<String> for ThoughtId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AreaOfLifeId(pub String);

impl From<String> for AreaOfLifeId {
    fn from(id: String) -> Self {
        Self(id)
    }
}
