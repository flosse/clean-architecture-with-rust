use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Thought {
    pub(crate) thought_id: String,
    pub(crate) title: String,
    pub(crate) areas_of_life: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaOfLife {
    pub(crate) area_of_life_id: String,
    pub(crate) name: String,
}
