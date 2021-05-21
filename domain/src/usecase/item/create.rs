pub struct Request {
    /// The title of new item.
    pub title: String,
}

pub struct Response<Id> {
    /// The ID of the newly created item.
    pub id: Id,
}

/// Create a new item with the given title.
pub trait CreateItem {
    type Err;
    type Id;
    fn exec(&self, req: Request) -> Result<Response<Self::Id>, Self::Err>;
}
