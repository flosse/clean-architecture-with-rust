//! All value objects and information that
//! belong to [AreaOfLife]s.

use crate::value_object;

mod name;
pub use self::name::*;

pub type Id = value_object::Id<AreaOfLife>;

/// An area of your life
#[derive(Debug, Clone)]
pub struct AreaOfLife {
    pub id: Id,
    pub name: Name,
}
