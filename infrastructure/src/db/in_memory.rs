use adapter::model::app::{thought::Id, NewId};
use application::gateway::repository::thought::{GetAllError, GetError, Repo, SaveError};
use entity::thought::Thought;
use std::{collections::HashMap, sync::RwLock};

#[derive(Default)]
pub struct InMemory {
    thoughts: RwLock<HashMap<Id, Thought>>,
}

impl Repo for InMemory {
    type Id = Id;
    fn save(&self, thought: Thought) -> Result<Self::Id, SaveError> {
        let id = self.new_id()?;
        self.thoughts.write().unwrap().insert(id, thought);
        Ok(id)
    }
    fn get(&self, id: Self::Id) -> Result<Thought, GetError> {
        self.thoughts
            .read()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or(GetError::NotFound)
    }
    fn get_all(&self) -> Result<Vec<(Self::Id, Thought)>, GetAllError> {
        Ok(self.thoughts.read().unwrap().clone().into_iter().collect())
    }
}

impl NewId<Id> for InMemory {
    type Err = SaveError;
    fn new_id(&self) -> Result<Id, Self::Err> {
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
