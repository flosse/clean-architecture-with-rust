use crate::{
    controller,
    model::app::{area_of_life as aol, thought},
    presenter::Present,
};
use cawr_application::{gateway::repository as repo, identifier::NewId};
use std::{collections::HashSet, sync::Arc};

pub struct Api<D, P> {
    db: Arc<D>,
    presenter: P,
}

impl<D, P> Clone for Api<D, P>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        let db = Arc::clone(&self.db);
        let presenter = self.presenter.clone();
        Self { db, presenter }
    }
}

impl<D, P> Api<D, P>
where
    D: repo::thought::Repo
        + repo::area_of_life::Repo
        + 'static
        + NewId<cawr_domain::thought::Id>
        + NewId<cawr_domain::area_of_life::Id>,
    P: Present<thought::create::Result>
        + Present<thought::delete::Result>
        + Present<thought::find_by_id::Result>
        + Present<thought::read_all::Result>
        + Present<thought::update::Result>
        + Present<aol::create::Result>
        + Present<aol::delete::Result>
        + Present<aol::read_all::Result>
        + Present<aol::update::Result>,
{
    pub const fn new(db: Arc<D>, presenter: P) -> Self {
        Self { db, presenter }
    }
    fn thought_controller(&self) -> controller::thought::Controller<D, P> {
        controller::thought::Controller::new(&self.db, &self.presenter)
    }
    fn aol_controller(&self) -> controller::area_of_life::Controller<D, P> {
        controller::area_of_life::Controller::new(&self.db, &self.presenter)
    }
    pub fn create_thought(
        &self,
        title: impl Into<String>,
        areas_of_life: &HashSet<String>,
    ) -> <P as Present<thought::create::Result>>::ViewModel {
        self.thought_controller()
            .create_thought(title, areas_of_life)
    }
    pub fn update_thought(
        &self,
        id: &str,
        title: impl Into<String>,
        areas_of_life: &HashSet<String>,
    ) -> <P as Present<thought::update::Result>>::ViewModel {
        self.thought_controller()
            .update_thought(id, title, areas_of_life)
    }
    pub fn delete_thought(&self, id: &str) -> <P as Present<thought::delete::Result>>::ViewModel {
        self.thought_controller().delete_thought(id)
    }
    pub fn find_thought(&self, id: &str) -> <P as Present<thought::find_by_id::Result>>::ViewModel {
        self.thought_controller().find_thought(id)
    }
    pub fn read_all_thoughts(&self) -> <P as Present<thought::read_all::Result>>::ViewModel {
        self.thought_controller().read_all_thoughts()
    }
    pub fn create_area_of_life(
        &self,
        name: impl Into<String>,
    ) -> <P as Present<aol::create::Result>>::ViewModel {
        self.aol_controller().create_area_of_life(name)
    }
    pub fn update_area_of_life(
        &self,
        id: &str,
        name: impl Into<String>,
    ) -> <P as Present<aol::update::Result>>::ViewModel {
        self.aol_controller().update_area_of_life(id, name)
    }
    pub fn delete_area_of_life(&self, id: &str) -> <P as Present<aol::delete::Result>>::ViewModel {
        self.aol_controller().delete_area_of_life(id)
    }
    pub fn read_all_areas_of_life(&self) -> <P as Present<aol::read_all::Result>>::ViewModel {
        self.aol_controller().read_all_areas_of_life()
    }
}
