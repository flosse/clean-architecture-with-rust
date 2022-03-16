use super::*;
use adapter::model::app::{area_of_life as aol, thought as app};
use application::{
    gateway::repository::thought::{DeleteError, GetAllError, GetError, Record, Repo, SaveError},
    identifier::{NewId, NewIdError},
};
use domain::thought::{Id, Thought, Title};
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
        let thought_id = thought.id().to_string();
        let title = String::from(thought.title().as_ref());
        let areas_of_life = thought
            .areas_of_life()
            .iter()
            .map(ToString::to_string)
            .collect();
        let model = models::Thought {
            thought_id,
            title,
            areas_of_life,
        };

        match self.storage_id(thought.id(), MAP_THOUGHT_ID_KEY) {
            Ok(storage_id) => {
                log::debug!("Update thought {}", thought.id());
                let sid = self
                    .thoughts
                    .save_with_id(&model, &storage_id)
                    .map_err(|err| {
                        log::warn!("Unable to save thought: {}", err);
                        SaveError::Connection
                    })?;
                debug_assert_eq!(sid, storage_id);
            }
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => {
                    log::debug!("Create new thought record");
                    let storage_id = self.thoughts.save(&model).map_err(|err| {
                        log::warn!("Unable to save thought: {}", err);
                        SaveError::Connection
                    })?;
                    self.save_id(storage_id, thought.id(), MAP_THOUGHT_ID_KEY)
                        .map_err(|err| {
                            log::warn!("Unable to save thought ID: {}", err);
                            SaveError::Connection
                        })?;
                }
                _ => {
                    return Err(SaveError::Connection);
                }
            },
        }

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
        let areas_of_life = model
            .areas_of_life
            .into_iter()
            .filter_map(|id| {
                id.parse::<aol::Id>()
                    .map_err(|err| {
                        log::warn!("{}", err);
                    })
                    .map(Into::into)
                    .ok()
            })
            .collect();
        Ok(Record {
            thought: Thought::new(id, Title::new(model.title), areas_of_life),
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
                let areas_of_life = model
                    .areas_of_life
                    .into_iter()
                    .filter_map(|id| {
                        id.parse::<aol::Id>()
                            .map_err(|err| {
                                log::warn!("{}", err);
                            })
                            .map(Into::into)
                            .ok()
                    })
                    .collect();

                model
                    .thought_id
                    .parse::<app::Id>()
                    .ok()
                    .map(Into::into)
                    .map(|id| (id, model.title, areas_of_life))
            })
            .map(|(id, title, areas_of_life)| Thought::new(id, Title::new(title), areas_of_life))
            .map(|thought| Record { thought })
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
