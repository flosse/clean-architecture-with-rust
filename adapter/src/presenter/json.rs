use crate::{id::thought::Id, presenter::Presenter as PresenterTrait};
use application::usecase::thought::{create, find_by_id};
use serde::Serialize;

#[derive(Default)]
pub struct Presenter;

#[derive(Debug, Serialize)]
pub struct Thought {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct ThoughtId(String);

impl PresenterTrait<create::Response<Id>> for Presenter {
    type ViewModel = ThoughtId;
    fn present(&self, data: create::Response<Id>) -> Self::ViewModel {
        ThoughtId(data.id.to_string())
    }
}

impl PresenterTrait<find_by_id::Response<Id>> for Presenter {
    type ViewModel = Thought;
    fn present(&self, data: find_by_id::Response<Id>) -> Self::ViewModel {
        let find_by_id::Response { id, title } = data;
        let id = id.to_string();
        Self::ViewModel { id, title }
    }
}
