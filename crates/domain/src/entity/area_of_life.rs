//! All value objects and information that
//! belong to [`AreaOfLife`]s.

use crate::value_object;

pub type Id = value_object::Id<AreaOfLife>;
pub type Name = value_object::Name<AreaOfLife>;

/// An area of your life
#[derive(Debug, Clone)]
pub struct AreaOfLife {
    id: Id,
    name: Name,
}

impl AreaOfLife {
    #[must_use]
    pub fn new(id: Id, name: Name) -> Self {
        // Never construct an area of life with invalid name
        debug_assert!(name.as_ref().len() <= Name::max_len());
        debug_assert!(name.as_ref().len() >= Name::min_len());
        Self { id, name }
    }
    #[must_use]
    pub const fn id(&self) -> Id {
        self.id
    }
    #[must_use]
    pub const fn name(&self) -> &Name {
        &self.name
    }
}

const MAX_NAME_LEN: usize = 30;
const MIN_NAME_LEN: usize = 5;

impl Name {
    pub const fn min_len() -> usize {
        MIN_NAME_LEN
    }
    pub const fn max_len() -> usize {
        MAX_NAME_LEN
    }
}
