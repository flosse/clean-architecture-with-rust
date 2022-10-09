use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    NameMinLength { min: usize, actual: usize },
    NameMaxLength { max: usize, actual: usize },
}
