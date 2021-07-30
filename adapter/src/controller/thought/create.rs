use crate::{
    model::app::thought::{create as app, Id},
    presenter::Present,
};
use application::{gateway::repository::thought::Repo, usecase::thought::create as uc};
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

    pub fn create_thought(&self, title: impl Into<String>) -> P::ViewModel {
        let title = title.into();
        log::debug!("Create thought '{}'", title);
        let req = app::Request { title };
        let interactor = uc::CreateThought::new(&*self.repository);
        let res = interactor.exec(req);
        self.presenter.present(res)
    }
}
