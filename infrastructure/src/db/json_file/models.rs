use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Thought {
    pub(crate) thought_id: String,
    pub(crate) title: String,
}

#[derive(Serialize, Deserialize)]
pub struct AreaOfLife {
    pub(crate) area_of_life_id: String,
    pub(crate) name: String,
}
