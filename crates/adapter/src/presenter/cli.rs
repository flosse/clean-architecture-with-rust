use crate::{
    model::app::{area_of_life, thought},
    presenter::Present,
};

#[derive(Default)]
pub struct Presenter;

impl Present<thought::create::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: thought::create::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("Created a new thought (ID = {})", data.id),
            Err(err) => format!("Undable to create a new thought: {}", err),
        }
    }
}

impl Present<thought::update::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: thought::update::Result) -> Self::ViewModel {
        match result {
            Ok(_) => "Updated thought".to_string(),
            Err(err) => format!("Undable to update thought: {}", err),
        }
    }
}

impl Present<thought::find_by_id::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: thought::find_by_id::Result) -> Self::ViewModel {
        match result {
            Ok(thought) => format!("{} ({})", thought.title, thought.id),
            Err(err) => format!("Unable find thought: {}", err),
        }
    }
}

impl Present<thought::read_all::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: thought::read_all::Result) -> Self::ViewModel {
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

impl Present<thought::delete::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: thought::delete::Result) -> Self::ViewModel {
        match result {
            Ok(_) => "Successfully deleted thought".to_string(),
            Err(err) => format!("Unable delete thought: {}", err),
        }
    }
}

impl Present<area_of_life::create::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: area_of_life::create::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("Created a new area of life (ID = {})", data.id),
            Err(err) => format!("Undable to create a new area of life: {}", err),
        }
    }
}

impl Present<area_of_life::update::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: area_of_life::update::Result) -> Self::ViewModel {
        match result {
            Ok(_) => "Updated area of life".to_string(),
            Err(err) => format!("Undable to update area of life: {}", err),
        }
    }
}

impl Present<area_of_life::read_all::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: area_of_life::read_all::Result) -> Self::ViewModel {
        match result {
            Ok(resp) => resp
                .areas_of_life
                .into_iter()
                .map(|t| format!("- {} ({})", t.name, t.id))
                .collect::<Vec<_>>()
                .join("\n"),
            Err(err) => format!("Unable read all areas of life: {}", err),
        }
    }
}

impl Present<area_of_life::delete::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: area_of_life::delete::Result) -> Self::ViewModel {
        match result {
            Ok(_) => "Successfully deleted area of life".to_string(),
            Err(err) => format!("Unable delete aref of life: {}", err),
        }
    }
}
