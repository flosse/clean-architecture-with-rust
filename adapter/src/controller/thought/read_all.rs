use crate::{
    model::app::thought::{read_all as app, Id},
    presenter::Present,
};
use application::{gateway::repository::thought::Repo, usecase::thought::read_all as uc};
use std::sync::Arc;

pub struct Controller<R, P> {
    repository: Arc<R>,
    presenter: P,
}

impl<R, P> Controller<R, P>
where
    R: Repo<Id = Id> + 'static,
    P: Present<app::Result>,
{
    pub fn new(repository: Arc<R>, presenter: P) -> Self {
        Self {
            repository,
            presenter,
        }
    }
    pub fn read_all(&self) -> P::ViewModel {
        log::debug!("Read all thoughts");
        let interactor = uc::ReadAll::new(&*self.repository);
        let res = interactor.exec(app::Request {});
        self.presenter.present(res)
    }
}
