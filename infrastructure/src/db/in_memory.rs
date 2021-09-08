use adapter::{db::Db, model::app};
use application::{
    gateway::repository::{area_of_life::AreaOfLifeRecord, thought::ThoughtRecord},
    identifier::{NewId, NewIdError},
};
use std::{collections::HashMap, sync::RwLock};

#[derive(Default)]
pub struct InMemory {
    thoughts: RwLock<HashMap<app::thought::Id, ThoughtRecord<app::thought::Id>>>,
    areas_of_life: RwLock<HashMap<app::area_of_life::Id, AreaOfLifeRecord<app::area_of_life::Id>>>,
}

impl Db for InMemory {}

mod thought {
    use super::*;
    use adapter::model::app::thought::Id;
    use application::gateway::repository::thought::{
        DeleteError, GetAllError, GetError, Repo, SaveError, ThoughtRecord,
    };

    impl Repo for InMemory {
        type Id = Id;
        fn save(&self, record: ThoughtRecord<Self::Id>) -> Result<(), SaveError> {
            self.thoughts.write().unwrap().insert(record.id, record);
            Ok(())
        }
        fn get(&self, id: Self::Id) -> Result<ThoughtRecord<Self::Id>, GetError> {
            self.thoughts
                .read()
                .unwrap()
                .get(&id)
                .cloned()
                .ok_or(GetError::NotFound)
        }
        fn get_all(&self) -> Result<Vec<ThoughtRecord<Self::Id>>, GetAllError> {
            Ok(self
                .thoughts
                .read()
                .unwrap()
                .iter()
                .map(|(_, r)| r)
                .cloned()
                .collect())
        }
        fn delete(&self, id: Self::Id) -> Result<(), DeleteError> {
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
                .map(|(id, _)| u32::from(*id))
                .max()
                .unwrap_or(0)
                + 1;
            Ok(Id::from(next))
        }
    }
}

mod area_of_life {
    use super::*;
    use adapter::model::app::area_of_life::Id;
    use application::gateway::repository::area_of_life::{
        AreaOfLifeRecord, DeleteError, GetAllError, GetError, Repo, SaveError,
    };

    impl Repo for InMemory {
        type Id = Id;
        fn save(&self, record: AreaOfLifeRecord<Self::Id>) -> Result<(), SaveError> {
            self.areas_of_life
                .write()
                .unwrap()
                .insert(record.id, record);
            Ok(())
        }
        fn get(&self, id: Self::Id) -> Result<AreaOfLifeRecord<Self::Id>, GetError> {
            self.areas_of_life
                .read()
                .unwrap()
                .get(&id)
                .cloned()
                .ok_or(GetError::NotFound)
        }
        fn get_all(&self) -> Result<Vec<AreaOfLifeRecord<Self::Id>>, GetAllError> {
            Ok(self
                .areas_of_life
                .read()
                .unwrap()
                .iter()
                .map(|(_, r)| r)
                .cloned()
                .collect())
        }
        fn delete(&self, id: Self::Id) -> Result<(), DeleteError> {
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
                .map(|(id, _)| u32::from(*id))
                .max()
                .unwrap_or(0)
                + 1;
            Ok(Id::from(next))
        }
    }
}
