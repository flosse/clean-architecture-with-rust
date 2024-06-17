use crate::{
    handler::{reply_error, reply_json, Result},
    AppApi,
};
use cawr_adapter::{db::Db, model::view::json::thought::update::Request};
use warp::Reply;

pub async fn handle<D>(id: String, req: Request, api: AppApi<D>) -> Result<impl Reply>
where
    D: Db,
{
    let areas_of_life = req
        .areas_of_life
        .into_iter()
        .map(|id| id.0.to_string())
        .collect();
    match api.update_thought(&id, req.title, &areas_of_life) {
        Ok(res) => Ok(reply_json(&res.data, res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}
