use adapter::id::{item::ItemId, NewId};
use application::gateway::repository::item::ItemRepo;
use entity::item::Item;
use std::{collections::HashMap, sync::Mutex};

#[derive(Default)]
pub struct InMemory {
    items: Mutex<HashMap<ItemId, Item>>,
}

pub enum Error {
    NotFound,
}

impl ItemRepo for InMemory {
    type Err = Error;
    type Id = ItemId;
    fn save(&self, item: Item) -> Result<Self::Id, Self::Err> {
        let id = self.new_id()?;
        self.items.lock().unwrap().insert(id, item);
        Ok(id)
    }
    fn get(&self, id: Self::Id) -> Result<Item, Self::Err> {
        self.items
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or(Error::NotFound)
    }
}

impl NewId<ItemId> for InMemory {
    type Err = Error;
    fn new_id(&self) -> Result<ItemId, Self::Err> {
        let next = self
            .items
            .lock()
            .unwrap()
            .iter()
            .map(|(id, _)| u32::from(*id))
            .max()
            .unwrap_or(0)
            + 1;
        Ok(ItemId::from(next))
    }
}
