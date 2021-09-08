pub mod thought {
    pub use json_boundary::{
        domain::{Thought, ThoughtId},
        usecase::thought::*,
    };
}
pub mod area_of_life {
    pub use json_boundary::{
        domain::{AreaOfLife, AreaOfLifeId},
        usecase::area_of_life::*,
    };
}
pub use json_boundary::{Error, Response, Result};
