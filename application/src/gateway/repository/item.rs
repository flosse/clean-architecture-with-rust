use entity::item::Item;

pub trait ItemRepo {
    type Err;
    type Id;
    fn save(&self, item: Item) -> Result<Self::Id, Self::Err>;
}
