use crate::{
    id::item::{ItemId, ParseItemIdError},
    presenter::Presenter,
};
use application::{
    gateway::repository::item::ItemRepo,
    usecase::item::find_by_id::{self, FindById, Request, Response},
};
use std::sync::Arc;
use thiserror::Error;

pub struct Controller<R, P> {
    repository: Arc<R>,
    presenter: P,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Parameter(#[from] ParseItemIdError),
    #[error(transparent)]
    Usecase(#[from] find_by_id::Error),
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
    pub fn find_item(&self, id: &str) -> Result<P::ViewModel, Error> {
        let interactor = FindById::new(&*self.repository);
        let req = Request { id: id.parse()? };
        let res = interactor.exec(req)?;
        Ok(self.presenter.present(res))
    }
}
