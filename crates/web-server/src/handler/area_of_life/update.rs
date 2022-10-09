use crate::{
    handler::{reply_error, Result},
    AppApi,
};
use cawr_adapter::{db::Db, model::view::json::area_of_life::update::Request};
use warp::{reply, Reply};

pub async fn handle<D>(id: String, req: Request, api: AppApi<D>) -> Result<impl Reply>
where
    D: Db,
{
    match api.update_area_of_life(&id, req.name) {
        Ok(res) => Ok(reply::with_status(reply::json(&res.data), res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}
