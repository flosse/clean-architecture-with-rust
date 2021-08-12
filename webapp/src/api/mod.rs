use json_boundary::{
    domain::{Thought, ThoughtId},
    usecase::thought::{create, delete, find_by_id, read_all},
};

mod http;

pub use http::{Error, Result};

pub async fn fetch_thought(id: &ThoughtId) -> Result<Thought, find_by_id::Error> {
    http::get_json(&format!("/api/thought/{}", id.0)).await
}

pub async fn fetch_all_thoughts() -> Result<Vec<Thought>, read_all::Error> {
    http::get_json("/api/thought").await
}

pub async fn create_thought(title: String) -> Result<ThoughtId, create::Error> {
    http::post_json("/api/thought", &create::Request { title }).await
}

pub async fn delete_thought(id: &ThoughtId) -> Result<(), delete::Error> {
    http::delete_json(&format!("/api/thought/{}", id.0), &()).await
}
