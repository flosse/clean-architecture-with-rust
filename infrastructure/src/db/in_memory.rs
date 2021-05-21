use application::gateway::repository::item::ItemRepo;
use entity::item::Item;
use std::{cell::RefCell, collections::HashMap, convert::Infallible};

#[derive(Default)]
pub struct InMemory {
    items: RefCell<HashMap<Id, Item>>,
}

type Id = u32;

impl ItemRepo for InMemory {
    type Err = Infallible;
    type Id = Id;
    fn save(&self, item: Item) -> Result<Self::Id, Self::Err> {
        let id = self
            .items
            .borrow()
            .iter()
            .map(|(id, _)| id)
            .max()
            .unwrap_or(&0)
            + 1;
        self.items.borrow_mut().insert(id, item);
        Ok(id)
    }
}
