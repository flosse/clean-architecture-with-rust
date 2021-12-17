//! All value objects and information that
//! belong to [AreaOfLife]s.

mod id;
mod name;

pub use self::{id::*, name::*};

/// An area of your life
#[derive(Debug, Clone)]
pub struct AreaOfLife {
    pub id: Id,
    pub name: Name,
}
