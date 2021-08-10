use adapter::model::app::{thought::Id, NewId};
use application::gateway::repository::thought::{GetError, Repo, SaveError};
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
    pub fn try_new() -> Result<Self, io::Error> {
        let cfg = Config {
            single: true,
            pretty: true,
            ..Default::default()
        };
        let thoughts = Store::new_with_cfg("thoughts", cfg)?;
        let ids = Store::new_with_cfg("ids", cfg)?;
        Ok(Self { thoughts, ids })
    }
    fn save_thought_id(&self, storage_id: StorageId, id: Id) -> Result<(), io::Error> {
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
    fn storage_id(&self, id: Id) -> Result<StorageId, io::Error> {
        let id = id.to_string();
        self.ids
            .get::<HashMap<String, String>>(MAP_THOUGHT_ID_KEY)?
            .get(&id)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Storage ID not found"))
    }
}

impl NewId<Id> for JsonFile {
    type Err = SaveError;
    fn new_id(&self) -> Result<Id, Self::Err> {
        let id = match self.ids.get::<u32>(LAST_THOUGHT_ID_KEY) {
            Ok(id) => Ok(id),
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => Ok(0),
                _ => {
                    log::warn!("Unable to fetch last thought ID key: {}", err);
                    Err(SaveError::Connection)
                }
            },
        }?;
        let new_id = id + 1;
        self.ids
            .save_with_id(&new_id, LAST_THOUGHT_ID_KEY)
            .map_err(|err| {
                log::warn!("Unable to save new ID: {}", err);
                SaveError::Connection
            })?;
        Ok(Id::from(new_id))
    }
}

type StorageId = String;

impl Repo for JsonFile {
    type Id = Id;

    fn save(&self, thought: Thought) -> Result<Self::Id, SaveError> {
        log::debug!("Save thought {:?} to JSON file", thought);
        let id = self.new_id()?;
        let Thought { title } = thought;
        let model = models::Thought {
            thought_id: id.to_string(),
            title: title.into_string(),
        };
        let storage_id = self.thoughts.save(&model).map_err(|err| {
            log::warn!("Unable to save thought: {}", err);
            SaveError::Connection
        })?;
        self.save_thought_id(storage_id, id).map_err(|err| {
            log::warn!("Unable to save thought ID: {}", err);
            SaveError::Connection
        })?;
        Ok(id)
    }
    fn get(&self, id: Self::Id) -> Result<Thought, GetError> {
        log::debug!("Get thought {:?} from JSON file", id);
        let sid = self.storage_id(id).map_err(|err| {
            log::warn!("Unable to get thought ID: {}", err);
            if err.kind() == io::ErrorKind::NotFound {
                GetError::NotFound
            } else {
                GetError::Connection
            }
        })?;
        let model = self.thoughts.get::<models::Thought>(&sid).map_err(|err| {
            log::warn!("Unable to fetch thought: {}", err);
            if err.kind() == io::ErrorKind::NotFound {
                GetError::NotFound
            } else {
                GetError::Connection
            }
        })?;
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
