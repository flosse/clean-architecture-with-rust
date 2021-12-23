use super::*;
use adapter::model::app::thought as app;
use application::{
    gateway::repository::thought::{DeleteError, GetAllError, GetError, Record, Repo, SaveError},
    identifier::{NewId, NewIdError},
};
use domain::thought::Id;
use domain::thought::{Thought, Title};
use std::io;

impl NewId<Id> for JsonFile {
    fn new_id(&self) -> Result<Id, NewIdError> {
        let id = self.new_id(LAST_THOUGHT_ID_KEY)?;
        Ok(id)
    }
}

impl Repo for JsonFile {
    fn save(&self, record: Record) -> Result<(), SaveError> {
        log::debug!("Save thought {:?} to JSON file", record);
        let Record { thought } = record;
        let Thought { id, title } = thought;
        let model = models::Thought {
            thought_id: id.to_string(),
            title: String::from(title),
        };
        let storage_id = self.thoughts.save(&model).map_err(|err| {
            log::warn!("Unable to save thought: {}", err);
            SaveError::Connection
        })?;
        self.save_id(storage_id, id, MAP_THOUGHT_ID_KEY)
            .map_err(|err| {
                log::warn!("Unable to save thought ID: {}", err);
                SaveError::Connection
            })?;
        Ok(())
    }
    fn get(&self, id: Id) -> Result<Record, GetError> {
        log::debug!("Get thought {:?} from JSON file", id);
        let sid = self.storage_id(id, MAP_THOUGHT_ID_KEY).map_err(|err| {
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
        Ok(Record {
            thought: Thought {
                id,
                title: Title::new(model.title),
            },
        })
    }
    fn get_all(&self) -> Result<Vec<Record>, GetAllError> {
        log::debug!("Get all thoughts from JSON file");
        let thoughts = self
            .thoughts
            .all::<models::Thought>()
            .map_err(|err| {
                log::warn!("Unable to load all thoughts: {}", err);
                GetAllError::Connection
            })?
            .into_iter()
            .filter_map(|(_, model)| {
                model
                    .thought_id
                    .parse::<app::Id>()
                    .ok()
                    .map(Into::into)
                    .map(|id| (id, model.title))
            })
            .map(|(id, title)| Record {
                thought: Thought {
                    id,
                    title: Title::new(title),
                },
            })
            .collect();
        Ok(thoughts)
    }
    fn delete(&self, id: Id) -> Result<(), DeleteError> {
        log::debug!("Delete thought {:?} from JSON file", id);
        let sid = self.storage_id(id, MAP_THOUGHT_ID_KEY).map_err(|err| {
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
