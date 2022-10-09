use crate::handler::{reply_error, Result};
use cawr_adapter::{
    controller::area_of_life::Controller, db::Db, model::view::json::area_of_life::update::Request,
    presenter::http_json_api::Presenter,
};
use std::sync::Arc;
use warp::{reply, Reply};

pub async fn handle<D>(
    id: String,
    req: Request,
    controller: Arc<Controller<D, Presenter>>,
) -> Result<impl Reply>
where
    D: Db,
{
    match controller.update_area_of_life(&id, req.name) {
        Ok(res) => Ok(reply::with_status(reply::json(&res.data), res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}
