//! All value objects and information that
//! belong to [Thought]s.

use crate::{entity::area_of_life as aol, value_object};
use std::collections::HashSet;

pub type Id = value_object::Id<Thought>;
pub type Title = value_object::Name<Thought>;

/// Anything you want to remember
#[derive(Debug, Clone)]
pub struct Thought {
    id: Id,
    title: Title,
    areas_of_life: HashSet<aol::Id>,
}

impl Thought {
    #[must_use]
    pub fn new(id: Id, title: Title, areas_of_life: HashSet<aol::Id>) -> Self {
        // Never construct a thought with invalid title
        debug_assert!(title.as_ref().len() <= Title::max_len());
        debug_assert!(title.as_ref().len() >= Title::min_len());
        Self {
            id,
            title,
            areas_of_life,
        }
    }
    #[must_use]
    pub const fn id(&self) -> Id {
        self.id
    }
    #[must_use]
    pub const fn title(&self) -> &Title {
        &self.title
    }
    #[must_use]
    pub const fn areas_of_life(&self) -> &HashSet<aol::Id> {
        &self.areas_of_life
    }
}

const MAX_TITLE_LEN: usize = 80;
const MIN_TITLE_LEN: usize = 3;

impl Title {
    pub const fn min_len() -> usize {
        MIN_TITLE_LEN
    }
    pub const fn max_len() -> usize {
        MAX_TITLE_LEN
    }
}
