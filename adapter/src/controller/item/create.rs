use crate::{id::item::ItemId, presenter::Presenter};
use application::{
    gateway::repository::item::ItemRepo,
    usecase::item::create::{self, CreateItem, Request, Response},
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
    Usecase(#[from] create::Error),
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

    pub fn create_item(&self, title: impl Into<String>) -> Result<P::ViewModel, Error> {
        let interactor = CreateItem::new(&*self.repository);
        let req = Request {
            title: title.into(),
        };
        let res = interactor.exec(req)?;
        Ok(self.presenter.present(res))
    }
}
