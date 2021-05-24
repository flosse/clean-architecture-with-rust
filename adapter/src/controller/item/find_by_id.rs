use crate::{
    id::item::{ItemId, ParseItemIdError},
    presenter::Presenter,
};
use application::{
    gateway::repository::item::ItemRepo,
    usecase::item::find_by_id::{self, FindById, Request, Response},
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
    Id(ParseItemIdError),
    #[error(transparent)]
    Usecase(find_by_id::Error<R>),
}

impl<R> fmt::Debug for Error<R>
where
    R: ItemRepo,
    RepoError<R>: error::Error,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Id(e) => e.fmt(f),
            Self::Usecase(e) => e.fmt(f),
        }
    }
}

impl<R, P> Controller<R, P>
where
    R: ItemRepo<Id = ItemId> + 'static,
    P: Presenter<Response<ItemId>>,
{
    pub fn new(repository: Arc<R>, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }
    pub fn find_item(&self, id: &str) -> Result<P::ViewModel, Error<R>>
    where
        RepoError<R>: error::Error + fmt::Debug + 'static,
    {
        let interactor = FindById::new(&*self.repository);
        let req = Request {
            id: id.parse().map_err(Error::Id)?,
        };
        let res = interactor.exec(req).map_err(Error::Usecase)?;
        Ok(self.presenter.present(res))
    }
}
