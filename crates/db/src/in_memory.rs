use std::collections::HashMap;

use parking_lot::RwLock;

use cawr_adapter::db::Db;
use cawr_application::{
    gateway::repository::{
        area_of_life::Record as AreaOfLifeRecord, thought::Record as ThoughtRecord,
    },
    identifier::{NewId, NewIdError},
};

#[derive(Default)]
pub struct InMemory {
    thoughts: RwLock<HashMap<cawr_domain::thought::Id, ThoughtRecord>>,
    areas_of_life: RwLock<HashMap<cawr_domain::area_of_life::Id, AreaOfLifeRecord>>,
}

impl Db for InMemory {}

mod thought {
    use super::*;
    use cawr_application::gateway::repository::thought::{
        DeleteError, GetAllError, GetError, Record, Repo, SaveError,
    };
    use cawr_domain::thought::Id;

    impl Repo for InMemory {
        fn save(&self, record: Record) -> Result<(), SaveError> {
            self.thoughts.write().insert(record.thought.id(), record);
            Ok(())
        }
        fn get(&self, id: Id) -> Result<Record, GetError> {
            self.thoughts
                .read()
                .get(&id)
                .cloned()
                .ok_or(GetError::NotFound)
        }
        fn get_all(&self) -> Result<Vec<Record>, GetAllError> {
            Ok(self
                .thoughts
                .read()
                .iter()
                .map(|(_, r)| r)
                .cloned()
                .collect())
        }
        fn delete(&self, id: Id) -> Result<(), DeleteError> {
            self.thoughts
                .write()
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
    use cawr_application::gateway::repository::area_of_life::{
        DeleteError, GetAllError, GetError, Record, Repo, SaveError,
    };
    use cawr_domain::area_of_life::Id;

    impl Repo for InMemory {
        fn save(&self, record: Record) -> Result<(), SaveError> {
            self.areas_of_life
                .write()
                .insert(record.area_of_life.id(), record);
            Ok(())
        }
        fn get(&self, id: Id) -> Result<Record, GetError> {
            self.areas_of_life
                .read()
                .get(&id)
                .cloned()
                .ok_or(GetError::NotFound)
        }
        fn get_all(&self) -> Result<Vec<Record>, GetAllError> {
            Ok(self
                .areas_of_life
                .read()
                .iter()
                .map(|(_, r)| r)
                .cloned()
                .collect())
        }
        fn delete(&self, id: Id) -> Result<(), DeleteError> {
            self.areas_of_life
                .write()
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
                .iter()
                .map(|(id, _)| id.to_u64())
                .max()
                .unwrap_or(0)
                + 1;
            Ok(Id::from(next))
        }
    }
}
