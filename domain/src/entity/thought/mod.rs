//! All value objects and information that
//! belong to [Thought]s.

mod id;
mod title;

use crate::entity::area_of_life as aol;
use std::collections::HashSet;

pub use self::{id::*, title::*};

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
