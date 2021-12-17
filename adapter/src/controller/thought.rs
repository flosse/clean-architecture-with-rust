use crate::{
    model::app::thought::{self as app, Id},
    presenter::Present,
};
use application::{gateway::repository::thought::Repo, identifier::NewId, usecase::thought as uc};
use std::sync::Arc;

pub struct Controller<D, P> {
    db: Arc<D>,
    presenter: P,
}

impl<D, P> Controller<D, P>
where
    D: Repo + 'static + NewId<domain::thought::Id>,
    P: Present<app::create::Result>
        + Present<app::delete::Result>
        + Present<app::find_by_id::Result>
        + Present<app::read_all::Result>,
{
    pub fn new(db: Arc<D>, presenter: P) -> Self {
        Self { db, presenter }
    }
    pub fn create_thought(
        &self,
        title: impl Into<String>,
    ) -> <P as Present<app::create::Result>>::ViewModel {
        let title = title.into();
        log::debug!("Create thought '{}'", title);
        let req = app::create::Request { title };
        let interactor = uc::create::CreateThought::new(&*self.db, &*self.db);
        let res = interactor.exec(req);
        self.presenter.present(res)
    }
    pub fn delete_thought(&self, id: &str) -> <P as Present<app::delete::Result>>::ViewModel {
        log::debug!("Delete thought {}", id);
        let res = id
            .parse::<Id>()
            .map_err(|_| app::delete::Error::Id)
            .map(Into::into)
            .map(|id| app::delete::Request { id })
            .and_then(|req| {
                let interactor = uc::delete::Delete::new(&*self.db);
                interactor.exec(req).map_err(|e| {
                    // TODO: impl From<uc::Error> for app::Error
                    match e {
                        uc::delete::Error::Repo => app::delete::Error::Repo,
                        uc::delete::Error::NotFound => app::delete::Error::NotFound,
                    }
                })
            });
        self.presenter.present(res)
    }
    pub fn find_thought(&self, id: &str) -> <P as Present<app::find_by_id::Result>>::ViewModel {
        log::debug!("Find thought {}", id);
        let res = id
            .parse::<Id>()
            .map_err(|_| app::find_by_id::Error::Id)
            .map(Into::into)
            .map(|id| app::find_by_id::Request { id })
            .and_then(|req| {
                let interactor = uc::find_by_id::FindById::new(&*self.db);
                interactor.exec(req).map_err(|e| {
                    // TODO: impl From<uc::Error> for app::Error
                    match e {
                        uc::find_by_id::Error::Repo => app::find_by_id::Error::Repo,
                        uc::find_by_id::Error::NotFound => app::find_by_id::Error::NotFound,
                    }
                })
            });
        self.presenter.present(res)
    }
    pub fn read_all_thoughts(&self) -> <P as Present<app::read_all::Result>>::ViewModel {
        log::debug!("Read all thoughts");
        let interactor = uc::read_all::ReadAll::new(&*self.db);
        let res = interactor.exec(app::read_all::Request {});
        self.presenter.present(res)
    }
}
