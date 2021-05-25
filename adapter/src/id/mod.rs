pub mod thought;

/// A service that generates a new entity ID.
pub trait NewId<T> {
    type Err;
    fn new_id(&self) -> Result<T, Self::Err>;
}
