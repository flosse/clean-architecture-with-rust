use json_boundary::{
    domain::{Thought, ThoughtId},
    usecase::thought::{create, find_by_id},
};

mod http;

pub use http::{Error, Result};

pub async fn fetch_thought(id: ThoughtId) -> Result<Thought, find_by_id::Error> {
    http::get_json(&format!("/api/thought/{}", id.0)).await
}

pub async fn create_thought(title: String) -> Result<ThoughtId, create::Error> {
    http::post_json("/api/thought", &create::Request { title }).await
}
