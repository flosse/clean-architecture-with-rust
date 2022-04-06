use crate::{
    api,
    domain::{AreaOfLifeId, Thought, ThoughtId},
    usecase::{ErrorPresenter, Present},
};

// ------ ------
//  Controller
// ------ ------

pub async fn create(title: String, areas_of_life: Vec<AreaOfLifeId>) -> Result<ThoughtId, String> {
    let presenter = ErrorPresenter::default();
    api::create_thought(title, areas_of_life)
        .await
        .map_err(|e| presenter.present(e))
}

pub async fn update(thought: Thought) -> Result<(), String> {
    let Thought {
        id,
        title,
        areas_of_life,
    } = thought;
    let presenter = ErrorPresenter::default();
    api::update_thought(id, title, areas_of_life)
        .await
        .map_err(|e| presenter.present(e))
}

pub async fn find_by_id(id: &ThoughtId) -> Result<Thought, String> {
    let presenter = ErrorPresenter::default();
    api::fetch_thought(id)
        .await
        .map_err(|e| presenter.present(e))
}

pub async fn fetch_all() -> Result<Vec<Thought>, String> {
    let presenter = ErrorPresenter::default();
    api::fetch_all_thoughts()
        .await
        .map_err(|e| presenter.present(e))
}

pub async fn delete(id: &ThoughtId) -> Result<(), String> {
    let presenter = ErrorPresenter::default();
    api::delete_thought(id)
        .await
        .map_err(|e| presenter.present(e))
}
