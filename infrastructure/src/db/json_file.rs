use adapter::id::{item::ItemId, NewId};
use application::gateway::repository::item::{Error, ItemRepo, Result};
use entity::item::{Item, Title};
use jfs::{Config, Store};
use std::{collections::HashMap, io};

pub struct JsonFile {
    items: Store,
    ids: Store,
}

const LAST_ITEM_ID_KEY: &str = "last-item-id";
const MAP_ITEM_ID_KEY: &str = "map-item-id";

impl JsonFile {
    pub fn try_new() -> Result<Self> {
        let cfg = Config {
            single: true,
            pretty: true,
            ..Default::default()
        };
        let items = Store::new_with_cfg("items", cfg)?;
        let ids = Store::new_with_cfg("ids", cfg)?;
        Ok(Self { items, ids })
    }
    fn save_item_id(&self, storage_id: StorageId, item_id: ItemId) -> Result<()> {
        let mut map = match self.ids.get::<HashMap<String, String>>(MAP_ITEM_ID_KEY) {
            Ok(map) => Ok(map),
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => Ok(HashMap::new()),
                _ => Err(err),
            },
        }?;
        map.insert(item_id.to_string(), storage_id);
        self.ids.save_with_id(&map, MAP_ITEM_ID_KEY)?;
        Ok(())
    }
    fn storage_id(&self, item_id: ItemId) -> Result<StorageId> {
        let id = item_id.to_string();
        self.ids
            .get::<HashMap<String, String>>(MAP_ITEM_ID_KEY)?
            .get(&id)
            .cloned()
            .ok_or(Error::NotFound)
    }
}

impl NewId<ItemId> for JsonFile {
    type Err = Error;
    fn new_id(&self) -> Result<ItemId> {
        let id = match self.ids.get::<u32>(LAST_ITEM_ID_KEY) {
            Ok(id) => Ok(id),
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => Ok(0),
                _ => Err(err),
            },
        }?;
        let new_id = id + 1;
        self.ids.save_with_id(&new_id, LAST_ITEM_ID_KEY)?;
        Ok(ItemId::from(new_id))
    }
}

type StorageId = String;

impl ItemRepo for JsonFile {
    type Id = ItemId;

    fn save(&self, item: Item) -> Result<Self::Id> {
        let item_id = self.new_id()?;
        let Item { title } = item;
        let model = models::Item {
            item_id: item_id.to_string(),
            title: title.into_string(),
        };
        let storage_id = self.items.save(&model)?;
        self.save_item_id(storage_id, item_id)?;
        Ok(item_id)
    }
    fn get(&self, id: Self::Id) -> Result<Item> {
        let sid = self.storage_id(id)?;
        let model = self.items.get::<models::Item>(&sid)?;
        debug_assert_eq!(id.to_string(), model.item_id);
        Ok(Item {
            title: Title::new(model.title),
        })
    }
}

mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Item {
        pub(crate) item_id: String,
        pub(crate) title: String,
    }
}
