use crate::db::in_memory::InMemory;
use adapter::model::app::{thought::Id, NewId};
use anyhow::Result;
use application::gateway::repository::thought::{self as repo, Repo};
use entity::thought::Thought;
use serde::Deserialize;
use std::sync::Arc;
use warp::reply::Response;

pub fn blank_db() -> Arc<InMemory> {
    Arc::new(InMemory::default())
}

pub fn corrupt_db() -> Arc<CorruptTestDb> {
    Arc::new(CorruptTestDb::default())
}

#[derive(Default)]
pub struct CorruptTestDb;

impl Repo for CorruptTestDb {
    type Id = Id;
    fn save(&self, _: Thought) -> Result<Self::Id, repo::SaveError> {
        Err(repo::SaveError::Connection)
    }
    fn get(&self, _: Self::Id) -> Result<Thought, repo::GetError> {
        Err(repo::GetError::Connection)
    }
    fn get_all(&self) -> Result<Vec<(Id, Thought)>, repo::GetAllError> {
        Err(repo::GetAllError::Connection)
    }
}

impl NewId<Id> for CorruptTestDb {
    type Err = repo::SaveError;
    fn new_id(&self) -> Result<Id, Self::Err> {
        Err(repo::SaveError::Connection)
    }
}

pub async fn response_json_body<T>(mut res: Response) -> Result<T>
where
    for<'de> T: Deserialize<'de>,
{
    let body = res.body_mut();
    let bytes = hyper::body::to_bytes(body).await?;
    let json = serde_json::from_slice(&bytes)?;
    Ok(json)
}

pub fn add_thought_to_db(db: &Arc<InMemory>, title: &str) {
    use entity::thought::*;
    let thought = Thought {
        title: Title::new(title.to_string()),
    };
    db.as_ref().save(thought).unwrap();
}
