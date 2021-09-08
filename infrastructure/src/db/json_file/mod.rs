use adapter::db::Db;
use application::identifier::NewIdError;
use jfs::{Config, Store};
use std::{collections::HashMap, io};

mod area_of_life;
mod models;
mod thought;

const LAST_THOUGHT_ID_KEY: &str = "last-thought-id";
const LAST_AREA_OF_LIFE_ID_KEY: &str = "last-area-of-life-id";
const MAP_THOUGHT_ID_KEY: &str = "map-thought-id";
const MAP_AREA_OF_LIFE_ID_KEY: &str = "map-area-of-life-id";

pub struct JsonFile {
    thoughts: Store,
    areas_of_life: Store,
    ids: Store,
}

impl JsonFile {
    pub fn try_new() -> Result<Self, io::Error> {
        let cfg = Config {
            single: true,
            pretty: true,
            ..Default::default()
        };
        let thoughts = Store::new_with_cfg("thoughts", cfg)?;
        let areas_of_life = Store::new_with_cfg("areas-of-life", cfg)?;
        let ids = Store::new_with_cfg("ids", cfg)?;
        Ok(Self {
            thoughts,
            areas_of_life,
            ids,
        })
    }
    fn save_id<I>(&self, storage_id: StorageId, id: I, key: &str) -> Result<(), io::Error>
    where
        I: ToString,
    {
        let mut map = match self.ids.get::<HashMap<String, String>>(key) {
            Ok(map) => Ok(map),
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => Ok(HashMap::new()),
                _ => Err(err),
            },
        }?;
        map.insert(id.to_string(), storage_id);
        self.ids.save_with_id(&map, key)?;
        Ok(())
    }
    fn storage_id<I>(&self, id: I, key: &str) -> Result<StorageId, io::Error>
    where
        I: ToString,
    {
        let id = id.to_string();
        self.ids
            .get::<HashMap<String, String>>(key)?
            .get(&id)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Storage ID not found"))
    }
    fn new_id<I>(&self, key: &str) -> Result<I, NewIdError>
    where
        I: From<u32>,
    {
        let id = match self.ids.get::<u32>(key) {
            Ok(id) => Ok(id),
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => Ok(0),
                _ => {
                    log::warn!("Unable to fetch last ID key: {}", err);
                    Err(NewIdError)
                }
            },
        }?;
        let new_id = id + 1;
        self.ids.save_with_id(&new_id, key).map_err(|err| {
            log::warn!("Unable to save new ID: {}", err);
            NewIdError
        })?;
        Ok(I::from(new_id))
    }
}

type StorageId = String;

impl Db for JsonFile {}
