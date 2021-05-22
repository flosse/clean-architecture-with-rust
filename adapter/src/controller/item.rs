use crate::presenter::item::Presenter;
use application::{
    gateway::repository::item::ItemRepo,
    usecase::item::create::{CreateItem, Request},
};
use std::{error, fmt};
use thiserror::Error;

pub struct ItemController<R, P> {
    repository: R,
    presenter: P,
}

type RepoError<R> = <R as ItemRepo>::Err;
type Id<R> = <R as ItemRepo>::Id;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Repo(Box<dyn error::Error>),
}

impl<R, P> ItemController<R, P>
where
    R: ItemRepo + 'static,
    P: Presenter<Id<R>>,
{
    pub fn new(repository: R, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub fn create_item(&self, title: impl Into<String>) -> Result<P::Out, Error>
    where
        RepoError<R>: error::Error + fmt::Debug + 'static,
    {
        let interactor = CreateItem::new(&self.repository);
        let req = Request {
            title: title.into(),
        };
        let res = interactor.exec(req).map_err(|e| Error::Repo(Box::new(e)))?;
        Ok(self.presenter.present(res))
    }
}
