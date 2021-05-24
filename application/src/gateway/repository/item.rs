use entity::item::Item;

pub trait ItemRepo: Send + Sync {
    type Err;
    type Id;
    fn save(&self, item: Item) -> Result<Self::Id, Self::Err>;
    fn get(&self, id: Self::Id) -> Result<Item, Self::Err>;
}
