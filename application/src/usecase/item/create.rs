use crate::gateway::repository::item::ItemRepo;
use entity::item::Item;
use std::{error, fmt};
use thiserror::Error;

pub struct Request {
    /// The title of new item.
    pub title: String,
}

pub struct Response<Id> {
    /// The ID of the newly created item.
    pub id: Id,
}

/// Create item usecase interactor
pub struct CreateItem<'r, R> {
    repo: &'r R,
}

impl<'r, R> CreateItem<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

type RepoError<R> = <R as ItemRepo>::Err;
type Id<R> = <R as ItemRepo>::Id;

#[derive(Error)]
pub enum Error<R>
where
    R: ItemRepo,
    RepoError<R>: error::Error + 'static,
{
    #[error(transparent)]
    Repo(RepoError<R>),
}

impl<R> fmt::Debug for Error<R>
where
    R: ItemRepo,
    RepoError<R>: error::Error,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Repo(e) => e.fmt(f),
        }
    }
}

impl<'r, R> CreateItem<'r, R>
where
    R: ItemRepo,
    RepoError<R>: error::Error,
{
    /// Create a new item with the given title.
    pub fn exec(&self, req: Request) -> Result<Response<Id<R>>, Error<R>> {
        let item = Item { title: req.title };
        let id = self.repo.save(item).map_err(Error::Repo)?;
        Ok(Response { id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default)]
    struct MockRepo {
        item: RefCell<Option<Item>>,
    }

    #[derive(Debug, Error)]
    enum Err {}

    impl ItemRepo for MockRepo {
        type Err = Err;
        type Id = u32;

        fn save(&self, item: Item) -> Result<Self::Id, Self::Err> {
            *self.item.borrow_mut() = Some(item);
            Ok(42)
        }
    }

    #[test]
    fn create_new_item() {
        let repo = MockRepo::default();
        let usecase = CreateItem::new(&repo);
        let req = Request {
            title: "foo".into(),
        };
        let res = usecase.exec(req).unwrap();
        assert_eq!(repo.item.borrow().as_ref().unwrap().title, "foo");
        assert_eq!(res.id, 42);
    }
}
