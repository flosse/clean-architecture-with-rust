use crate::{model::app::thought as app, presenter::Present};

#[derive(Default)]
pub struct Presenter;

impl Present<app::create::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: app::create::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("Created a new thought (ID = {})", data.id.to_string()),
            Err(err) => format!("Undable to create a new thought: {}", err),
        }
    }
}

impl Present<app::find_by_id::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: app::find_by_id::Result) -> Self::ViewModel {
        match result {
            Ok(thought) => format!("{} ({})", thought.title, thought.id.to_string()),
            Err(err) => format!("Unable find thought: {}", err),
        }
    }
}
