use adapter::model::app::thought::Id;
use application::gateway::repository::thought::{
    DeleteError, GetAllError, GetError, NewId, NewIdError, Repo, SaveError, ThoughtRecord,
};
use std::{collections::HashMap, sync::RwLock};

#[derive(Default)]
pub struct InMemory {
    thoughts: RwLock<HashMap<Id, ThoughtRecord<Id>>>,
}

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
