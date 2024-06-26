use crate::{
    handler::{reply_error, reply_json, Result},
    AppApi,
};
use cawr_adapter::db::Db;
use warp::Reply;

pub async fn handle<D>(api: AppApi<D>) -> Result<impl Reply>
where
    D: Db,
{
    match api.read_all_areas_of_life() {
        Ok(res) => Ok(reply_json(&res.data, res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}
