use adapter::db::Db;
use application::identifier::NewIdError;
use jfs::{Config, Store};
use std::{collections::HashMap, fs, io, path::Path};

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
    pub fn try_new<P: AsRef<Path>>(dir: P) -> Result<Self, io::Error> {
        let cfg = Config {
            single: true,
            pretty: true,
            ..Default::default()
        };
        let dir = dir.as_ref();
        fs::create_dir_all(dir)?;
        let thoughts = Store::new_with_cfg(dir.join("thoughts"), cfg)?;
        let areas_of_life = Store::new_with_cfg(dir.join("areas-of-life"), cfg)?;
        let ids = Store::new_with_cfg(dir.join("ids"), cfg)?;
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
        I: From<u64>,
    {
        let id = match self.ids.get::<u64>(key) {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    mod area_of_life {
        use super::*;
        use domain::{
            area_of_life::{AreaOfLife, Id as AolId, Name},
            thought::{Id as ThoughtId, Thought, Title},
        };
        use std::collections::HashSet;
        use tempdir::TempDir;

        #[test]
        fn delete_references_in_thoughts() {
            use application::{
                gateway::repository::{
                    area_of_life::{Record as AolRecord, Repo as AolRepo},
                    thought::{Record as ThoughtRecord, Repo as ThoughtRepo},
                },
                identifier::NewId,
            };
            // -- setup --
            init();
            let test_dir = TempDir::new("tests").unwrap();
            log::debug!("Test directory: {}", test_dir.path().display());
            let db = JsonFile::try_new(&test_dir).unwrap();
            let aol_id = (&db as &dyn NewId<AolId>).new_id().unwrap();
            let name = Name::new("test aol".to_string());
            let area_of_life = AreaOfLife { id: aol_id, name };
            let record = AolRecord { area_of_life };
            (&db as &dyn AolRepo).save(record).unwrap();
            let mut areas_of_life = HashSet::new();
            areas_of_life.insert(aol_id);
            let id = (&db as &dyn NewId<ThoughtId>).new_id().unwrap();
            let thought = Thought::new(id, Title::new("foo".to_string()), areas_of_life);
            let record = ThoughtRecord { thought };
            (&db as &dyn ThoughtRepo).save(record).unwrap();
            // -- test --
            (&db as &dyn AolRepo).delete(aol_id).unwrap();
            let rec = (&db as &dyn ThoughtRepo).get(id).unwrap();
            assert!(rec.thought.areas_of_life().is_empty());
        }
    }
}
