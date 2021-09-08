use crate::web::handler::{reply_error, Result};
use adapter::{controller::area_of_life::Controller, db::Db, presenter::http_json_api::Presenter};
use std::sync::Arc;
use warp::{reply, Reply};

pub type Request = String;

pub async fn handle<D>(
    req: Request,
    controller: Arc<Controller<D, Presenter>>,
) -> Result<impl Reply>
where
    D: Db,
{
    match controller.delete_area_of_life(&req) {
        Ok(res) => Ok(reply::with_status(reply::json(&res.data), res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}
