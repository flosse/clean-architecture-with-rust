//! A generalised ID type for entities and aggregates.

use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[derive(Debug)]
pub struct Id<T> {
    id: u64,
    // The `fn() -> T` is a trick to tell the compiler that we don't own anything.
    marker: PhantomData<fn() -> T>,
}

impl<T> Id<T> {
    pub const fn new(id: u64) -> Self {
        Self {
            id,
            marker: PhantomData,
        }
    }
    pub const fn to_u64(self) -> u64 {
        self.id
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self::new(self.id)
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(hasher)
    }
}

impl<T> From<u64> for Id<T> {
    fn from(from: u64) -> Self {
        Self::new(from)
    }
}

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_id() {
        let id: Id<()> = Id::new(33);
        assert_eq!(format!("{}", id), "33");
    }
}
