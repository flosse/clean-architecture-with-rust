use crate::{id::item::ItemId, presenter::Presenter as PresenterTrait};
use application::usecase::item::{create, find_by_id};

#[derive(Default)]
pub struct Presenter;

impl PresenterTrait<create::Response<ItemId>> for Presenter {
    type ViewModel = String;
    fn present(&self, data: create::Response<ItemId>) -> Self::ViewModel {
        data.id.to_string()
    }
}

impl PresenterTrait<find_by_id::Response<ItemId>> for Presenter {
    type ViewModel = String;
    fn present(&self, data: find_by_id::Response<ItemId>) -> Self::ViewModel {
        format!("{} ({})", data.title, data.id.to_string())
    }
}
