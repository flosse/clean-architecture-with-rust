use crate::{
    api,
    domain::{AreaOfLife, AreaOfLifeId},
    usecase::{ErrorPresenter, Present},
};

// ------ ------
//  Controller
// ------ ------

pub async fn create(name: String) -> Result<AreaOfLifeId, String> {
    let presenter = ErrorPresenter::default();
    api::create_area_of_life(name)
        .await
        .map_err(|e| presenter.present(e))
}

pub async fn fetch_all() -> Result<Vec<AreaOfLife>, String> {
    let presenter = ErrorPresenter::default();
    api::fetch_all_areas_of_life()
        .await
        .map_err(|e| presenter.present(e))
}

pub async fn delete(id: &AreaOfLifeId) -> Result<(), String> {
    let presenter = ErrorPresenter::default();
    api::delete_area_of_life(id)
        .await
        .map_err(|e| presenter.present(e))
}
