use super::*;
use adapter::model::app::area_of_life::Id;
use application::{
    gateway::repository::area_of_life::{
        AreaOfLifeRecord, DeleteError, GetAllError, GetError, Repo, SaveError,
    },
    identifier::{NewId, NewIdError},
};
use domain::area_of_life::{AreaOfLife, Name};
use std::io;

impl NewId<Id> for JsonFile {
    fn new_id(&self) -> Result<Id, NewIdError> {
        let id = self.new_id(LAST_AREA_OF_LIFE_ID_KEY)?;
        Ok(id)
    }
}

impl Repo for JsonFile {
    type Id = Id;

    fn save(&self, record: AreaOfLifeRecord<Self::Id>) -> Result<(), SaveError> {
        log::debug!("Save area of life {:?} to JSON file", record);
        let AreaOfLifeRecord { area_of_life, id } = record;
        let AreaOfLife { name } = area_of_life;
        let model = models::AreaOfLife {
            area_of_life_id: id.to_string(),
            name: String::from(name),
        };
        let storage_id = self.areas_of_life.save(&model).map_err(|err| {
            log::warn!("Unable to save area of life: {}", err);
            SaveError::Connection
        })?;
        self.save_id(storage_id, id, MAP_AREA_OF_LIFE_ID_KEY)
            .map_err(|err| {
                log::warn!("Unable to save area of life ID: {}", err);
                SaveError::Connection
            })?;
        Ok(())
    }
    fn get(&self, id: Self::Id) -> Result<AreaOfLifeRecord<Self::Id>, GetError> {
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
        Ok(AreaOfLifeRecord {
            id,
            area_of_life: AreaOfLife {
                name: Name::new(model.name),
            },
        })
    }
    fn get_all(&self) -> Result<Vec<AreaOfLifeRecord<Self::Id>>, GetAllError> {
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
                    .parse()
                    .ok()
                    .map(|id| (id, model.name))
            })
            .map(|(id, name)| AreaOfLifeRecord {
                id,
                area_of_life: AreaOfLife {
                    name: Name::new(name),
                },
            })
            .collect();
        Ok(areas_of_life)
    }
    fn delete(&self, id: Self::Id) -> Result<(), DeleteError> {
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
        Ok(())
    }
}
