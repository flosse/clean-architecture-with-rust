//! All value objects and information that
//! belong to [AreaOfLife]s.

use crate::value_object;

pub type Id = value_object::Id<AreaOfLife>;
pub type Name = value_object::Name<AreaOfLife>;

/// An area of your life
#[derive(Debug, Clone)]
pub struct AreaOfLife {
    pub id: Id,
    pub name: Name,
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
