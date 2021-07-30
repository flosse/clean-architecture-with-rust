use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Thought {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct ThoughtId(pub String);
