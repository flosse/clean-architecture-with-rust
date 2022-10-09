use crate::{
    handler::{reply_error, Result},
    AppApi,
};
use cawr_adapter::db::Db;
use warp::{reply, Reply};

pub type Request = String;

pub async fn handle<D>(req: Request, api: AppApi<D>) -> Result<impl Reply>
where
    D: Db,
{
    match api.delete_area_of_life(&req) {
        Ok(res) => Ok(reply::with_status(reply::json(&res.data), res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}
