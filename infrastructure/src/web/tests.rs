use crate::db::in_memory::InMemory;
use adapter::db::Db;
use anyhow::Result;
use application::{
    gateway::repository::thought::Record as ThoughtRecord,
    identifier::{NewId, NewIdError},
};
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

impl Db for CorruptTestDb {}

mod thought {
    use super::*;
    use application::gateway::repository::thought::{self as repo, Record, Repo};
    use domain::thought::Id;

    impl Repo for CorruptTestDb {
        fn save(&self, _: Record) -> Result<(), repo::SaveError> {
            Err(repo::SaveError::Connection)
        }
        fn get(&self, _: Id) -> Result<Record, repo::GetError> {
            Err(repo::GetError::Connection)
        }
        fn get_all(&self) -> Result<Vec<Record>, repo::GetAllError> {
            Err(repo::GetAllError::Connection)
        }
        fn delete(&self, _: Id) -> Result<(), repo::DeleteError> {
            Err(repo::DeleteError::Connection)
        }
    }

    impl NewId<Id> for CorruptTestDb {
        fn new_id(&self) -> Result<Id, NewIdError> {
            Err(NewIdError)
        }
    }
}

mod area_of_life {
    use super::*;
    use application::gateway::repository::area_of_life::{self as repo, Record, Repo};
    use domain::area_of_life::Id;

    impl Repo for CorruptTestDb {
        fn save(&self, _: Record) -> Result<(), repo::SaveError> {
            Err(repo::SaveError::Connection)
        }
        fn get(&self, _: Id) -> Result<Record, repo::GetError> {
            Err(repo::GetError::Connection)
        }
        fn get_all(&self) -> Result<Vec<Record>, repo::GetAllError> {
            Err(repo::GetAllError::Connection)
        }
        fn delete(&self, _: Id) -> Result<(), repo::DeleteError> {
            Err(repo::DeleteError::Connection)
        }
    }

    impl NewId<Id> for CorruptTestDb {
        fn new_id(&self) -> Result<Id, NewIdError> {
            Err(NewIdError)
        }
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
    use application::gateway::repository::thought::Repo;
    use domain::thought::*;

    let thought = ThoughtRecord {
        thought: Thought {
            id: db.new_id().unwrap(),
            title: Title::new(title.to_string()),
        },
    };
    db.as_ref().save(thought).unwrap();
}
