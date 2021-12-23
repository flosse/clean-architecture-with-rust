use adapter::db::Db;
use application::{
    gateway::repository::{
        area_of_life::Record as AreaOfLifeRecord, thought::Record as ThoughtRecord,
    },
    identifier::{NewId, NewIdError},
};
use std::{collections::HashMap, sync::RwLock};

#[derive(Default)]
pub struct InMemory {
    thoughts: RwLock<HashMap<domain::thought::Id, ThoughtRecord>>,
    areas_of_life: RwLock<HashMap<domain::area_of_life::Id, AreaOfLifeRecord>>,
}

impl Db for InMemory {}

mod thought {
    use super::*;
    use application::gateway::repository::thought::{
        DeleteError, GetAllError, GetError, Record, Repo, SaveError,
    };
    use domain::thought::Id;

    impl Repo for InMemory {
        fn save(&self, record: Record) -> Result<(), SaveError> {
            self.thoughts
                .write()
                .unwrap()
                .insert(record.thought.id, record);
            Ok(())
        }
        fn get(&self, id: Id) -> Result<Record, GetError> {
            self.thoughts
                .read()
                .unwrap()
                .get(&id)
                .cloned()
                .ok_or(GetError::NotFound)
        }
        fn get_all(&self) -> Result<Vec<Record>, GetAllError> {
            Ok(self
                .thoughts
                .read()
                .unwrap()
                .iter()
                .map(|(_, r)| r)
                .cloned()
                .collect())
        }
        fn delete(&self, id: Id) -> Result<(), DeleteError> {
            self.thoughts
                .write()
                .unwrap()
                .remove(&id)
                .map(|_| ())
                .ok_or(DeleteError::NotFound)
        }
    }

    impl NewId<Id> for InMemory {
        fn new_id(&self) -> Result<Id, NewIdError> {
            let next = self
                .thoughts
                .read()
                .unwrap()
                .iter()
                .map(|(id, _)| id.to_u64())
                .max()
                .unwrap_or(0)
                + 1;
            Ok(Id::from(next))
        }
    }
}

mod area_of_life {
    use super::*;
    use application::gateway::repository::area_of_life::{
        DeleteError, GetAllError, GetError, Record, Repo, SaveError,
    };
    use domain::area_of_life::Id;

    impl Repo for InMemory {
        fn save(&self, record: Record) -> Result<(), SaveError> {
            self.areas_of_life
                .write()
                .unwrap()
                .insert(record.area_of_life.id, record);
            Ok(())
        }
        fn get(&self, id: Id) -> Result<Record, GetError> {
            self.areas_of_life
                .read()
                .unwrap()
                .get(&id)
                .cloned()
                .ok_or(GetError::NotFound)
        }
        fn get_all(&self) -> Result<Vec<Record>, GetAllError> {
            Ok(self
                .areas_of_life
                .read()
                .unwrap()
                .iter()
                .map(|(_, r)| r)
                .cloned()
                .collect())
        }
        fn delete(&self, id: Id) -> Result<(), DeleteError> {
            self.areas_of_life
                .write()
                .unwrap()
                .remove(&id)
                .map(|_| ())
                .ok_or(DeleteError::NotFound)
        }
    }

    impl NewId<Id> for InMemory {
        fn new_id(&self) -> Result<Id, NewIdError> {
            let next = self
                .areas_of_life
                .read()
                .unwrap()
                .iter()
                .map(|(id, _)| id.to_u64())
                .max()
                .unwrap_or(0)
                + 1;
            Ok(Id::from(next))
        }
    }
}
