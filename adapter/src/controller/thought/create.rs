use crate::{
    model::app::thought::{create as app, Id},
    presenter::Present,
};
use application::{
    gateway::repository::thought::{NewId, Repo},
    usecase::thought::create as uc,
};
use std::sync::Arc;

pub struct Controller<R, G, P> {
    repository: Arc<R>,
    id_gen: Arc<G>,
    presenter: P,
}

impl<R, G, P> Controller<R, G, P>
where
    R: Repo<Id = Id> + 'static,
    G: NewId<Id>,
    P: Present<app::Result>,
{
    pub fn new(repository: Arc<R>, id_gen: Arc<G>, presenter: P) -> Self {
        Self {
            repository,
            id_gen,
            presenter,
        }
    }

    pub fn create_thought(&self, title: impl Into<String>) -> P::ViewModel {
        let title = title.into();
        log::debug!("Create thought '{}'", title);
        let req = app::Request { title };
        let interactor = uc::CreateThought::new(&*self.repository, &*self.id_gen);
        let res = interactor.exec(req);
        self.presenter.present(res)
    }
}
