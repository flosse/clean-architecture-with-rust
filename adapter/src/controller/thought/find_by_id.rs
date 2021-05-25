use crate::{
    id::thought::{Id, ParseError},
    presenter::Presenter,
};
use application::{
    gateway::repository::thought::Repo,
    usecase::thought::find_by_id::{self, FindById, Request, Response},
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
    Parameter(#[from] ParseError),
    #[error(transparent)]
    Usecase(#[from] find_by_id::Error),
}

impl<R, P> Controller<R, P>
where
    R: Repo<Id = Id> + 'static,
    P: Presenter<Response<Id>>,
{
    pub fn new(repository: Arc<R>, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }
    pub fn find_thought(&self, id: &str) -> Result<P::ViewModel, Error> {
        let interactor = FindById::new(&*self.repository);
        let req = Request { id: id.parse()? };
        let res = interactor.exec(req)?;
        Ok(self.presenter.present(res))
    }
}
