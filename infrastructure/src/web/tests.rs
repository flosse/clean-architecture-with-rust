use crate::db::in_memory::InMemory;
use anyhow::Result;
use application::gateway::repository::thought::{self as repo, Repo};
use entity::thought::Thought;
use serde::Deserialize;
use std::{io, sync::Arc};
use warp::reply::Response;

use adapter::id::{thought::Id, NewId};

pub fn blank_db() -> Arc<InMemory> {
    Arc::new(InMemory::default())
}

pub fn corrupt_db() -> Arc<CorruptTestDb> {
    Arc::new(CorruptTestDb::default())
}

#[derive(Default)]
pub struct CorruptTestDb;

fn io_err<T>() -> repo::Result<T> {
    Err(repo::Error::Io(io::Error::new(
        io::ErrorKind::Other,
        "no connection",
    )))
}

impl Repo for CorruptTestDb {
    type Id = Id;
    fn save(&self, _: Thought) -> repo::Result<Self::Id> {
        io_err()
    }
    fn get(&self, _: Self::Id) -> repo::Result<Thought> {
        io_err()
    }
}

impl NewId<Id> for CorruptTestDb {
    type Err = repo::Error;
    fn new_id(&self) -> repo::Result<Id> {
        io_err()
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
