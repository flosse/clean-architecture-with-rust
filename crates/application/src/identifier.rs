use thiserror::Error;

/// A service that generates a new entity ID.
// The creation of the ID should be done **before** we save a record.
// To do that we delegate the generation of a new ID to a separate
// service that can be injected e.g. into a specific usecase.
// See: https://matthiasnoback.nl/2018/05/when-and-where-to-determine-the-id-of-an-entity/
pub trait NewId<Id> {
    fn new_id(&self) -> Result<Id, NewIdError>;
}

#[derive(Debug, Error)]
#[error("Unable to generade a new entity ID")]
pub struct NewIdError;
