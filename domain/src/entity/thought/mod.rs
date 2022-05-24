//! All value objects and information that
//! belong to [Thought]s.

use crate::{entity::area_of_life as aol, value_object};
use std::collections::HashSet;

mod title;
pub use self::title::*;

pub type Id = value_object::Id<Thought>;

/// Anything you want to remember
#[derive(Debug, Clone)]
pub struct Thought {
    id: Id,
    title: Title,
    areas_of_life: HashSet<aol::Id>,
}

impl Thought {
    pub const fn new(id: Id, title: Title, areas_of_life: HashSet<aol::Id>) -> Self {
        Self {
            id,
            title,
            areas_of_life,
        }
    }
    pub const fn id(&self) -> Id {
        self.id
    }
    pub const fn title(&self) -> &Title {
        &self.title
    }
    pub const fn areas_of_life(&self) -> &HashSet<aol::Id> {
        &self.areas_of_life
    }
}
