use adapter::model::app::thought::Id;
use application::{
    gateway::repository::thought::{
        DeleteError, GetAllError, GetError, Repo, SaveError, ThoughtRecord,
    },
    identifier::{NewId, NewIdError},
};
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
    fn new_id(&self) -> Result<Id, NewIdError> {
        let id = match self.ids.get::<u32>(LAST_THOUGHT_ID_KEY) {
            Ok(id) => Ok(id),
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => Ok(0),
                _ => {
                    log::warn!("Unable to fetch last thought ID key: {}", err);
                    Err(NewIdError)
                }
            },
        }?;
        let new_id = id + 1;
        self.ids
            .save_with_id(&new_id, LAST_THOUGHT_ID_KEY)
            .map_err(|err| {
                log::warn!("Unable to save new ID: {}", err);
                NewIdError
            })?;
        Ok(Id::from(new_id))
    }
}

type StorageId = String;

impl Repo for JsonFile {
    type Id = Id;

    fn save(&self, record: ThoughtRecord<Self::Id>) -> Result<(), SaveError> {
        log::debug!("Save thought {:?} to JSON file", record);
        let ThoughtRecord { thought, id } = record;
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
        Ok(())
    }
    fn get(&self, id: Self::Id) -> Result<ThoughtRecord<Self::Id>, GetError> {
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
        Ok(ThoughtRecord {
            id,
            thought: Thought {
                title: Title::new(model.title),
            },
        })
    }
    fn get_all(&self) -> Result<Vec<ThoughtRecord<Self::Id>>, GetAllError> {
        log::debug!("Get all thoughts from JSON file");
        let thoughts = self
            .thoughts
            .all::<models::Thought>()
            .map_err(|err| {
                log::warn!("Unable to load all thoughts: {}", err);
                GetAllError::Connection
            })?
            .into_iter()
            .filter_map(|(_, model)| model.thought_id.parse().ok().map(|id| (id, model.title)))
            .map(|(id, title)| ThoughtRecord {
                id,
                thought: Thought {
                    title: Title::new(title),
                },
            })
            .collect();
        Ok(thoughts)
    }
    fn delete(&self, id: Self::Id) -> Result<(), DeleteError> {
        log::debug!("Delete thought {:?} from JSON file", id);
        let sid = self.storage_id(id).map_err(|err| {
            log::warn!("Unable to get thought ID: {}", err);
            if err.kind() == io::ErrorKind::NotFound {
                DeleteError::NotFound
            } else {
                DeleteError::Connection
            }
        })?;
        self.thoughts.delete(&sid).map_err(|err| {
            log::warn!("Unable to delete thought: {}", err);
            if err.kind() == io::ErrorKind::NotFound {
                DeleteError::NotFound
            } else {
                DeleteError::Connection
            }
        })?;
        Ok(())
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
