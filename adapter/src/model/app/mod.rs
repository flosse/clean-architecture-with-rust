pub mod thought;

/// A service that generates a new entity ID.
pub trait NewId<Id> {
    type Err;
    fn new_id(&self) -> Result<Id, Self::Err>;
}
