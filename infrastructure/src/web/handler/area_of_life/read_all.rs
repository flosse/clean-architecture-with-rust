use crate::web::handler::{reply_error, Result};
use adapter::{controller::area_of_life::Controller, db::Db, presenter::http_json_api::Presenter};
use std::sync::Arc;
use warp::{reply, Reply};

pub async fn handle<D>(controller: Arc<Controller<D, Presenter>>) -> Result<impl Reply>
where
    D: Db,
{
    match controller.read_all_areas_of_life() {
        Ok(res) => Ok(reply::with_status(reply::json(&res.data), res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}
