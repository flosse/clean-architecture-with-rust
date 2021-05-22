use application::gateway::repository::item::ItemRepo;
use entity::item::Item;
use jfs::{Config, Store};
use std::io;

pub struct JsonFile(Store);

impl JsonFile {
    pub fn try_new() -> Result<Self, io::Error> {
        let cfg = Config {
            single: true,
            pretty: true,
            ..Default::default()
        };
        let db = Store::new_with_cfg("items", cfg)?;
        Ok(Self(db))
    }
}

type Id = String;

impl ItemRepo for JsonFile {
    type Err = io::Error;
    type Id = Id;

    fn save(&self, item: Item) -> Result<Self::Id, Self::Err> {
        let item = models::Item::from(item);
        let id = self.0.save(&item)?;
        Ok(id)
    }
}

mod models {
    use entity::item as e;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Item {
        title: String,
    }

    impl From<e::Item> for Item {
        fn from(from: e::Item) -> Self {
            let e::Item { title } = from;
            Self { title }
        }
    }

    impl From<Item> for e::Item {
        fn from(from: Item) -> Self {
            let Item { title } = from;
            Self { title }
        }
    }
}
