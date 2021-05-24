use crate::{id::item::ItemId, presenter::item::create::Presenter};
use application::{
    gateway::repository::item::ItemRepo,
    usecase::item::create::{self, CreateItem, Request},
};
use std::{error, fmt, sync::Arc};
use thiserror::Error;

pub struct Controller<R, P> {
    repository: Arc<R>,
    presenter: P,
}

type RepoError<R> = <R as ItemRepo>::Err;

#[derive(Error)]
pub enum Error<R>
where
    R: ItemRepo + 'static,
    RepoError<R>: error::Error + 'static,
{
    #[error(transparent)]
    Usecase(create::Error<R>),
}

impl<R> fmt::Debug for Error<R>
where
    R: ItemRepo,
    RepoError<R>: error::Error,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Usecase(e) => e.fmt(f),
        }
    }
}

impl<R, P> Controller<R, P>
where
    R: ItemRepo<Id = ItemId> + 'static,
    P: Presenter,
{
    pub fn new(repository: Arc<R>, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub fn create_item(&self, title: impl Into<String>) -> Result<P::Out, Error<R>>
    where
        RepoError<R>: error::Error + fmt::Debug + 'static,
    {
        let interactor = CreateItem::new(&*self.repository);
        let req = Request {
            title: title.into(),
        };
        let res = interactor.exec(req).map_err(Error::Usecase)?;
        Ok(self.presenter.present(res))
    }
}
