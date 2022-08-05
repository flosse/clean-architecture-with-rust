use crate::handler::{reply_error, Result};
use adapter::{
    controller::thought::Controller, db::Db, model::view::json::thought::update::Request,
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
    let areas_of_life = req
        .areas_of_life
        .into_iter()
        .map(|id| id.0.to_string())
        .collect();
    match controller.update_thought(&id, req.title, &areas_of_life) {
        Ok(res) => Ok(reply::with_status(reply::json(&res.data), res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}
