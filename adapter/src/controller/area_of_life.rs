use crate::{
    model::app::area_of_life::{self as app, Id},
    presenter::Present,
};
use cawr_application::{
    gateway::repository::area_of_life::Repo, identifier::NewId, usecase::area_of_life as uc,
};
use cawr_domain::area_of_life as aol;
use std::sync::Arc;

pub struct Controller<D, P> {
    db: Arc<D>,
    presenter: P,
}

impl<D, P> Controller<D, P>
where
    D: Repo + 'static + NewId<aol::Id>,
    P: Present<app::create::Result>
        + Present<app::delete::Result>
        + Present<app::read_all::Result>
        + Present<app::update::Result>,
{
    pub fn new(db: Arc<D>, presenter: P) -> Self {
        Self { db, presenter }
    }
    pub fn create_area_of_life(
        &self,
        name: impl Into<String>,
    ) -> <P as Present<app::create::Result>>::ViewModel {
        let name = name.into();
        log::debug!("Create area of life '{}'", name);
        let req = app::create::Request { name };
        let interactor = uc::create::CreateAreaOfLife::new(&*self.db, &*self.db);
        let res = interactor.exec(req);
        self.presenter.present(res)
    }
    pub fn update_area_of_life(
        &self,
        id: &str,
        name: impl Into<String>,
    ) -> <P as Present<app::update::Result>>::ViewModel {
        let name = name.into();
        log::debug!("Update area of life '{:?}'", id);
        let res = id
            .parse::<Id>()
            .map_err(|_| app::update::Error::Id)
            .and_then(|id| {
                let req = app::update::Request {
                    id: id.into(),
                    name,
                };
                let interactor = uc::update::UpdateAreaOfLife::new(&*self.db);
                interactor.exec(req).map_err(Into::into)
            });
        self.presenter.present(res)
    }
    pub fn delete_area_of_life(&self, id: &str) -> <P as Present<app::delete::Result>>::ViewModel {
        log::debug!("Delete area of life {}", id);
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
    pub fn read_all_areas_of_life(&self) -> <P as Present<app::read_all::Result>>::ViewModel {
        log::debug!("Read all areas of life");
        let interactor = uc::read_all::ReadAll::new(&*self.db);
        let res = interactor.exec(app::read_all::Request {});
        self.presenter.present(res)
    }
}
