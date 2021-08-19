use crate::web::handler;
use adapter::model::app::thought::Id;
use application::gateway::repository::thought::{NewId, Repo};
use std::{convert::Infallible, sync::Arc};
use warp::{body, path, Filter, Rejection, Reply};

pub fn api<D>(db: Arc<D>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    D: Repo<Id = Id> + 'static + NewId<Id>,
{
    let base_path = path("api").and(path("thought"));

    // POST /api/thought
    let post_thought = warp::post()
        .and(path::end())
        .and(body::json())
        .and(with_db(db.clone()))
        .and_then(handler::thought::create::handle);

    // GET /api/thought
    let get_thoughts = warp::get()
        .and(path::end())
        .and(with_db(db.clone()))
        .and_then(handler::thought::read_all::handle);

    // GET /api/thought/<ID>
    let get_thought = warp::get()
        .and(path!(String))
        .and(path::end())
        .and(with_db(db.clone()))
        .and_then(handler::thought::find_by_id::handle);

    // DELETE /api/thought/<ID>
    let delete_thought = warp::delete()
        .and(path!(String))
        .and(path::end())
        .and(with_db(db))
        .and_then(handler::thought::delete::handle);

    base_path.and(
        post_thought
            .or(get_thoughts)
            .or(get_thought)
            .or(delete_thought),
    )
}

fn with_db<D>(db: Arc<D>) -> impl Filter<Extract = (Arc<D>,), Error = Infallible> + Clone
where
    D: Repo<Id = Id> + 'static,
{
    warp::any().map(move || db.clone())
}
