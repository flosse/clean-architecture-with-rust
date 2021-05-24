use crate::gateway::repository::item::{Error as RepoError, ItemRepo};
use domain::validate::item::{validate_item, ItemInvalidity};
use entity::item::{Item, Title};
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

type Id<R> = <R as ItemRepo>::Id;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Repo(#[from] RepoError),
    #[error(transparent)]
    Invalidity(#[from] ItemInvalidity),
}

impl<'r, R> CreateItem<'r, R>
where
    R: ItemRepo,
{
    /// Create a new item with the given title.
    pub fn exec(&self, req: Request) -> Result<Response<Id<R>>, Error> {
        let item = Item {
            title: Title(req.title),
        };
        validate_item(&item)?;
        let id = self.repo.save(item)?;
        Ok(Response { id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::RwLock;

    #[derive(Default)]
    struct MockRepo {
        item: RwLock<Option<Item>>,
    }

    impl ItemRepo for MockRepo {
        type Id = u32;

        fn save(&self, item: Item) -> Result<Self::Id, RepoError> {
            *self.item.write().unwrap() = Some(item);
            Ok(42)
        }
        fn get(&self, _: Self::Id) -> Result<Item, RepoError> {
            todo!()
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
        assert_eq!(repo.item.read().unwrap().as_ref().unwrap().title.0, "foo");
        assert_eq!(res.id, 42);
    }

    #[test]
    fn create_with_empty_title() {
        let repo = MockRepo::default();
        let usecase = CreateItem::new(&repo);
        let req = Request { title: "".into() };
        let err = usecase.exec(req).err().unwrap();
        assert!(matches!(err, Error::Invalidity(_)));
    }
}
