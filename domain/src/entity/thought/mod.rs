mod title; // Value object

pub use title::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Thought {
    pub title: Title,
}
