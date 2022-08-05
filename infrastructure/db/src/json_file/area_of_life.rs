use super::*;
use adapter::model::app::area_of_life as app;
use application::{
    gateway::repository::{
        area_of_life::{DeleteError, GetAllError, GetError, Record, Repo, SaveError},
        thought::Repo as ThoughtRepo,
    },
    identifier::{NewId, NewIdError},
};
use domain::{
    area_of_life::{AreaOfLife, Id, Name},
    thought::Thought,
};
use std::io;

impl NewId<Id> for JsonFile {
    fn new_id(&self) -> Result<Id, NewIdError> {
        let id = self.new_id(LAST_AREA_OF_LIFE_ID_KEY)?;
        Ok(id)
    }
}

impl Repo for JsonFile {
    fn save(&self, record: Record) -> Result<(), SaveError> {
        log::debug!("Save area of life {:?} to JSON file", record);
        let Record { area_of_life } = record;
        let name = area_of_life.name();
        let id = area_of_life.id();
        let model = models::AreaOfLife {
            area_of_life_id: id.to_string(),
            name: String::from(name.as_ref()),
        };

        match self.storage_id(area_of_life.id(), MAP_AREA_OF_LIFE_ID_KEY) {
            Ok(storage_id) => {
                log::debug!("Update area of life {}", area_of_life.id());
                let sid = self
                    .areas_of_life
                    .save_with_id(&model, &storage_id)
                    .map_err(|err| {
                        log::warn!("Unable to save area of life: {}", err);
                        SaveError::Connection
                    })?;
                debug_assert_eq!(sid, storage_id);
            }
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => {
                    log::debug!("Create new area of life record");
                    let storage_id = self.areas_of_life.save(&model).map_err(|err| {
                        log::warn!("Unable to save area of life: {}", err);
                        SaveError::Connection
                    })?;
                    self.save_id(storage_id, id, MAP_AREA_OF_LIFE_ID_KEY)
                        .map_err(|err| {
                            log::warn!("Unable to save area of life ID: {}", err);
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
        log::debug!("Get area of life {:?} from JSON file", id);
        let sid = self
            .storage_id(id, MAP_AREA_OF_LIFE_ID_KEY)
            .map_err(|err| {
                log::warn!("Unable to get area of life ID: {}", err);
                if err.kind() == io::ErrorKind::NotFound {
                    GetError::NotFound
                } else {
                    GetError::Connection
                }
            })?;
        let model = self
            .areas_of_life
            .get::<models::AreaOfLife>(&sid)
            .map_err(|err| {
                log::warn!("Unable to fetch area of life: {}", err);
                if err.kind() == io::ErrorKind::NotFound {
                    GetError::NotFound
                } else {
                    GetError::Connection
                }
            })?;
        debug_assert_eq!(id.to_string(), model.area_of_life_id);
        Ok(Record {
            area_of_life: AreaOfLife::new(id, Name::new(model.name)),
        })
    }
    fn get_all(&self) -> Result<Vec<Record>, GetAllError> {
        log::debug!("Get all areas of life from JSON file");
        let areas_of_life = self
            .areas_of_life
            .all::<models::AreaOfLife>()
            .map_err(|err| {
                log::warn!("Unable to load all areas of life: {}", err);
                GetAllError::Connection
            })?
            .into_iter()
            .filter_map(|(_, model)| {
                model
                    .area_of_life_id
                    .parse::<app::Id>()
                    .ok()
                    .map(Into::into)
                    .map(|id| (id, model.name))
            })
            .map(|(id, name)| Record {
                area_of_life: AreaOfLife::new(id, Name::new(name)),
            })
            .collect();
        Ok(areas_of_life)
    }
    fn delete(&self, id: Id) -> Result<(), DeleteError> {
        log::debug!("Delete area of life {:?} from JSON file", id);
        let sid = self
            .storage_id(id, MAP_AREA_OF_LIFE_ID_KEY)
            .map_err(|err| {
                log::warn!("Unable to get area of life ID: {}", err);
                if err.kind() == io::ErrorKind::NotFound {
                    DeleteError::NotFound
                } else {
                    DeleteError::Connection
                }
            })?;
        self.areas_of_life.delete(&sid).map_err(|err| {
            log::warn!("Unable to delete area of life: {}", err);
            if err.kind() == io::ErrorKind::NotFound {
                DeleteError::NotFound
            } else {
                DeleteError::Connection
            }
        })?;

        let thoughts = (self as &dyn ThoughtRepo).get_all().map_err(|err| {
            log::warn!("Unable to load thoughts: {}", err);
            DeleteError::Connection
        })?;

        log::debug!("Delete area of life {id} from thoughts");
        for mut rec in thoughts {
            if rec.thought.areas_of_life().iter().any(|x| x == &id) {
                log::debug!("Delete area of life {id} from {:?}", rec.thought);
                let mut areas_of_life = rec.thought.areas_of_life().clone();
                areas_of_life.retain(|x| x != &id);
                let updated_thought =
                    Thought::new(rec.thought.id(), rec.thought.title().clone(), areas_of_life);
                rec.thought = updated_thought;
                (self as &dyn ThoughtRepo).save(rec).map_err(|err| {
                    log::warn!("Unable to save thought: {}", err);
                    DeleteError::Connection
                })?;
            }
        }
        Ok(())
    }
}
