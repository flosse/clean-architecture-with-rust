use crate::{
    handler::{reply_error, Result},
    AppApi,
};
use cawr_adapter::{db::Db, model::view::json::area_of_life::create::Request};
use warp::{reply, Reply};

pub async fn handle<D>(req: Request, api: AppApi<D>) -> Result<impl Reply>
where
    D: Db,
{
    match api.create_area_of_life(req.name) {
        Ok(res) => Ok(reply::with_status(reply::json(&res.data), res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}
