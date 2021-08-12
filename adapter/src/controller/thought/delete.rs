use crate::{
    model::app::thought::{delete as app, Id},
    presenter::Present,
};
use application::{gateway::repository::thought::Repo, usecase::thought::delete as uc};
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
    pub fn delete_thought(&self, id: &str) -> P::ViewModel {
        log::debug!("Delete thought {}", id);
        let res = id
            .parse::<Id>()
            .map_err(|_| app::Error::Id)
            .map(|id| app::Request { id })
            .and_then(|req| {
                let interactor = uc::Delete::new(&*self.repository);
                interactor.exec(req).map_err(|e| {
                    // TODO: impl From<uc::Error> for app::Error
                    match e {
                        uc::Error::Repo => app::Error::Repo,
                        uc::Error::NotFound => app::Error::NotFound,
                    }
                })
            });
        self.presenter.present(res)
    }
}
