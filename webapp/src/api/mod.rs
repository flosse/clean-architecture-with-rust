mod http;

pub use self::{area_of_life::*, thought::*};
pub use http::{Error, Result};

mod thought {
    use crate::api::http::{self, Result};
    use cawr_json_boundary::{
        domain::{AreaOfLifeId, Thought, ThoughtId},
        usecase::thought::{create, delete, find_by_id, read_all, update},
    };

    pub async fn fetch_thought(id: &ThoughtId) -> Result<Thought, find_by_id::Error> {
        http::get_json(&format!("/api/thought/{}", id.0)).await
    }

    pub async fn fetch_all_thoughts() -> Result<Vec<Thought>, read_all::Error> {
        http::get_json("/api/thought").await
    }

    pub async fn create_thought(
        title: String,
        areas_of_life: Vec<AreaOfLifeId>,
    ) -> Result<ThoughtId, create::Error> {
        http::post_json(
            "/api/thought",
            &create::Request {
                title,
                areas_of_life,
            },
        )
        .await
    }

    pub async fn update_thought(
        id: ThoughtId,
        title: String,
        areas_of_life: Vec<AreaOfLifeId>,
    ) -> Result<(), update::Error> {
        http::put_json(
            &format!("/api/thought/{}", id),
            &update::Request {
                id,
                title,
                areas_of_life,
            },
        )
        .await
    }

    pub async fn delete_thought(id: &ThoughtId) -> Result<(), delete::Error> {
        http::delete_json(&format!("/api/thought/{}", id.0), &()).await
    }
}

mod area_of_life {
    use crate::api::http::{self, Result};
    use cawr_json_boundary::{
        domain::{AreaOfLife, AreaOfLifeId},
        usecase::area_of_life::{create, delete, read_all, update},
    };
    const RESOURCE: &str = "area-of-life";

    pub async fn fetch_all_areas_of_life() -> Result<Vec<AreaOfLife>, read_all::Error> {
        http::get_json(&format!("/api/{RESOURCE}")).await
    }

    pub async fn create_area_of_life(name: String) -> Result<AreaOfLifeId, create::Error> {
        http::post_json(&format!("/api/{RESOURCE}"), &create::Request { name }).await
    }

    pub async fn update_area_of_life(id: AreaOfLifeId, name: String) -> Result<(), update::Error> {
        http::put_json(
            &format!("/api/{RESOURCE}/{id}"),
            &update::Request { id, name },
        )
        .await
    }

    pub async fn delete_area_of_life(id: &AreaOfLifeId) -> Result<(), delete::Error> {
        http::delete_json(&format!("/api/{RESOURCE}/{id}"), &()).await
    }
}
