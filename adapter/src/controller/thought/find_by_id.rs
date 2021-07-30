use crate::{
    model::app::thought::{find_by_id as app, Id},
    presenter::Present,
};
use application::{gateway::repository::thought::Repo, usecase::thought::find_by_id as uc};
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
    pub fn find_thought(&self, id: &str) -> P::ViewModel {
        log::debug!("Find thought {}", id);
        let res = id
            .parse::<Id>()
            .map_err(app::Error::Id)
            .map(|id| app::Request { id })
            .and_then(|req| {
                let interactor = uc::FindById::new(&*self.repository);
                interactor.exec(req).map_err(app::Error::Repo)
            });
        self.presenter.present(res)
    }
}
