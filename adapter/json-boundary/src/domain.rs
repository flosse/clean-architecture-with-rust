use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Thought {
    pub id: ThoughtId,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThoughtId(pub String);

impl From<String> for ThoughtId {
    fn from(id: String) -> Self {
        Self(id)
    }
}
