use crate::{id::thought::Id, presenter::Presenter as PresenterTrait};
use application::usecase::thought::{create, find_by_id};

#[derive(Default)]
pub struct Presenter;

impl PresenterTrait<create::Response<Id>> for Presenter {
    type ViewModel = String;
    fn present(&self, data: create::Response<Id>) -> Self::ViewModel {
        data.id.to_string()
    }
}

impl PresenterTrait<find_by_id::Response<Id>> for Presenter {
    type ViewModel = String;
    fn present(&self, data: find_by_id::Response<Id>) -> Self::ViewModel {
        format!("{} ({})", data.title, data.id.to_string())
    }
}
