use crate::{model::app::thought as app, presenter::Present};

#[derive(Default)]
pub struct Presenter;

impl Present<app::create::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: app::create::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("Created a new thought (ID = {})", data.id),
            Err(err) => format!("Undable to create a new thought: {}", err),
        }
    }
}

impl Present<app::find_by_id::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: app::find_by_id::Result) -> Self::ViewModel {
        match result {
            Ok(thought) => format!("{} ({})", thought.title, thought.id),
            Err(err) => format!("Unable find thought: {}", err),
        }
    }
}

impl Present<app::read_all::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: app::read_all::Result) -> Self::ViewModel {
        match result {
            Ok(resp) => resp
                .thoughts
                .into_iter()
                .map(|t| format!("- {} ({})", t.title, t.id))
                .collect::<Vec<_>>()
                .join("\n"),
            Err(err) => format!("Unable read all thoughts: {}", err),
        }
    }
}

impl Present<app::delete::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: app::delete::Result) -> Self::ViewModel {
        match result {
            Ok(_) => "Successfully deleted thought".to_string(),
            Err(err) => format!("Unable delete thought: {}", err),
        }
    }
}
