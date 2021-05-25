use crate::{id::thought::Id, presenter::Presenter};
use application::{
    gateway::repository::thought::Repo,
    usecase::thought::create::{self, CreateThought, Request, Response},
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
    R: Repo<Id = Id> + 'static,
    P: Presenter<Response<Id>>,
{
    pub fn new(repository: Arc<R>, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }

    pub fn create_thought(&self, title: impl Into<String>) -> Result<P::ViewModel, Error> {
        let interactor = CreateThought::new(&*self.repository);
        let req = Request {
            title: title.into(),
        };
        let res = interactor.exec(req)?;
        Ok(self.presenter.present(res))
    }
}
