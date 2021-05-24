use crate::{id::item::ItemId, presenter::Presenter as PresenterTrait};
use application::usecase::item::{create, find_by_id};
use serde::Serialize;

#[derive(Default)]
pub struct Presenter;

#[derive(Debug, Serialize)]
pub struct Item {
    pub title: String,
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct Id(String);

impl PresenterTrait<create::Response<ItemId>> for Presenter {
    type ViewModel = Id;
    fn present(&self, data: create::Response<ItemId>) -> Self::ViewModel {
        Id(data.id.to_string())
    }
}

impl PresenterTrait<find_by_id::Response<ItemId>> for Presenter {
    type ViewModel = Item;
    fn present(&self, data: find_by_id::Response<ItemId>) -> Self::ViewModel {
        let find_by_id::Response { id, title } = data;
        let id = id.to_string();
        Self::ViewModel { id, title }
    }
}
