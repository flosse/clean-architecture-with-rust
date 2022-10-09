use crate::{ui::Msg, Api};
use cawr_adapter::db::Db;

pub fn read_all_areas_of_life<D>(api: Api<D>) -> Option<Msg>
where
    D: Db,
{
    match api.read_all_areas_of_life() {
        Ok(resp) => {
            let msg = Msg::AreasOfLifeChanged(resp.data.unwrap());
            return Some(msg);
        }
        Err(err) => {
            log::error!("Unable to read areas of life: {err:?}");
        }
    }
    None
}

pub fn read_all_thoughts<D>(api: Api<D>) -> Option<Msg>
where
    D: Db,
{
    match api.read_all_thoughts() {
        Ok(resp) => {
            let msg = Msg::ThoughtsChanged(resp.data.unwrap());
            return Some(msg);
        }
        Err(err) => {
            log::error!("Unable to read thoughts: {err:?}");
        }
    }
    None
}
