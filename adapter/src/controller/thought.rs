use crate::{
    model::app::{
        area_of_life as aol,
        thought::{self as app, Id},
    },
    presenter::Present,
};
use cawr_application::{gateway::repository as repo, identifier::NewId, usecase::thought as uc};
use std::{collections::HashSet, sync::Arc};

pub struct Controller<D, P> {
    db: Arc<D>,
    presenter: P,
}

impl<D, P> Controller<D, P>
where
    D: repo::thought::Repo + repo::area_of_life::Repo + 'static + NewId<cawr_domain::thought::Id>,
    P: Present<app::create::Result>
        + Present<app::delete::Result>
        + Present<app::find_by_id::Result>
        + Present<app::read_all::Result>
        + Present<app::update::Result>,
{
    pub fn new(db: Arc<D>, presenter: P) -> Self {
        Self { db, presenter }
    }
    pub fn create_thought(
        &self,
        title: impl Into<String>,
        areas_of_life: &HashSet<String>,
    ) -> <P as Present<app::create::Result>>::ViewModel {
        let title = title.into();
        log::debug!("Create thought '{}'", title);
        let res = parse_area_of_life_ids(areas_of_life)
            .map_err(Into::into)
            .and_then(|areas_of_life: HashSet<_>| {
                let req = app::create::Request {
                    title,
                    areas_of_life,
                };
                let interactor = uc::create::CreateThought::new(&*self.db, &*self.db);
                interactor.exec(req).map_err(Into::into)
            });
        self.presenter.present(res)
    }
    pub fn update_thought(
        &self,
        id: &str,
        title: impl Into<String>,
        areas_of_life: &HashSet<String>,
    ) -> <P as Present<app::update::Result>>::ViewModel {
        let title = title.into();
        log::debug!("Update thought '{:?}'", id);
        let res = id
            .parse::<Id>()
            .map_err(|_| app::update::Error::Id)
            .and_then(|id| {
                parse_area_of_life_ids(areas_of_life)
                    .map_err(Into::into)
                    .and_then(|areas_of_life: HashSet<_>| {
                        let req = app::update::Request {
                            id: id.into(),
                            title,
                            areas_of_life,
                        };
                        let interactor = uc::update::UpdateThought::new(&*self.db);
                        interactor.exec(req).map_err(Into::into)
                    })
            });
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
                interactor.exec(req).map_err(app::delete::Error::from)
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
                interactor.exec(req).map_err(app::find_by_id::Error::from)
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

fn parse_area_of_life_ids(
    areas_of_life: &HashSet<String>,
) -> Result<HashSet<cawr_domain::area_of_life::Id>, aol::ParseIdError> {
    areas_of_life
        .iter()
        .map(|id| id.parse())
        .collect::<Result<HashSet<aol::Id>, _>>()
        .map(|ids| ids.into_iter().map(Into::into).collect())
}
