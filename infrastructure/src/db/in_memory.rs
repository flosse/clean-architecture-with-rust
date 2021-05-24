use adapter::id::{item::ItemId, NewId};
use application::gateway::repository::item::{Error, ItemRepo, Result};
use entity::item::Item;
use std::{collections::HashMap, sync::RwLock};

#[derive(Default)]
pub struct InMemory {
    items: RwLock<HashMap<ItemId, Item>>,
}

impl ItemRepo for InMemory {
    type Id = ItemId;
    fn save(&self, item: Item) -> Result<Self::Id> {
        let id = self.new_id()?;
        self.items.write().unwrap().insert(id, item);
        Ok(id)
    }
    fn get(&self, id: Self::Id) -> Result<Item> {
        self.items
            .read()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or(Error::NotFound)
    }
}

impl NewId<ItemId> for InMemory {
    type Err = Error;
    fn new_id(&self) -> Result<ItemId> {
        let next = self
            .items
            .read()
            .unwrap()
            .iter()
            .map(|(id, _)| u32::from(*id))
            .max()
            .unwrap_or(0)
            + 1;
        Ok(ItemId::from(next))
    }
}
