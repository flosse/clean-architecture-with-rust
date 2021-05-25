use adapter::id::{thought::Id, NewId};
use application::gateway::repository::thought::{Error, Repo, Result};
use entity::thought::{Thought, Title};
use jfs::{Config, Store};
use std::{collections::HashMap, io};

pub struct JsonFile {
    thoughts: Store,
    ids: Store,
}

const LAST_THOUGHT_ID_KEY: &str = "last-thought-id";
const MAP_THOUGHT_ID_KEY: &str = "map-thought-id";

impl JsonFile {
    pub fn try_new() -> Result<Self> {
        let cfg = Config {
            single: true,
            pretty: true,
            ..Default::default()
        };
        let thoughts = Store::new_with_cfg("thoughts", cfg)?;
        let ids = Store::new_with_cfg("ids", cfg)?;
        Ok(Self { thoughts, ids })
    }
    fn save_thought_id(&self, storage_id: StorageId, id: Id) -> Result<()> {
        let mut map = match self.ids.get::<HashMap<String, String>>(MAP_THOUGHT_ID_KEY) {
            Ok(map) => Ok(map),
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => Ok(HashMap::new()),
                _ => Err(err),
            },
        }?;
        map.insert(id.to_string(), storage_id);
        self.ids.save_with_id(&map, MAP_THOUGHT_ID_KEY)?;
        Ok(())
    }
    fn storage_id(&self, id: Id) -> Result<StorageId> {
        let id = id.to_string();
        self.ids
            .get::<HashMap<String, String>>(MAP_THOUGHT_ID_KEY)?
            .get(&id)
            .cloned()
            .ok_or(Error::NotFound)
    }
}

impl NewId<Id> for JsonFile {
    type Err = Error;
    fn new_id(&self) -> Result<Id> {
        let id = match self.ids.get::<u32>(LAST_THOUGHT_ID_KEY) {
            Ok(id) => Ok(id),
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => Ok(0),
                _ => Err(err),
            },
        }?;
        let new_id = id + 1;
        self.ids.save_with_id(&new_id, LAST_THOUGHT_ID_KEY)?;
        Ok(Id::from(new_id))
    }
}

type StorageId = String;

impl Repo for JsonFile {
    type Id = Id;

    fn save(&self, thought: Thought) -> Result<Self::Id> {
        let id = self.new_id()?;
        let Thought { title } = thought;
        let model = models::Thought {
            thought_id: id.to_string(),
            title: title.into_string(),
        };
        let storage_id = self.thoughts.save(&model)?;
        self.save_thought_id(storage_id, id)?;
        Ok(id)
    }
    fn get(&self, id: Self::Id) -> Result<Thought> {
        let sid = self.storage_id(id)?;
        let model = self.thoughts.get::<models::Thought>(&sid)?;
        debug_assert_eq!(id.to_string(), model.thought_id);
        Ok(Thought {
            title: Title::new(model.title),
        })
    }
}

mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Thought {
        pub(crate) thought_id: String,
        pub(crate) title: String,
    }
}
