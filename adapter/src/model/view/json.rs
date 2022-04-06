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

mod conv {
    use super::*;
    use crate::model::app;

    impl From<app::area_of_life::Id> for area_of_life::AreaOfLifeId {
        fn from(from: app::area_of_life::Id) -> Self {
            from.to_u64().into()
        }
    }

    impl From<app::thought::Id> for thought::ThoughtId {
        fn from(from: app::thought::Id) -> Self {
            from.to_u64().into()
        }
    }
}
