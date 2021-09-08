mod name; // Value object

pub use self::name::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AreaOfLife {
    pub name: Name,
}
