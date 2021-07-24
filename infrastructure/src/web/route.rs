use crate::web::handler;
use adapter::id::thought::Id;
use application::gateway::repository::thought::Repo;
use std::{convert::Infallible, sync::Arc};
use warp::{body, path, Filter, Rejection, Reply};

pub fn api<D>(db: Arc<D>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    D: Repo<Id = Id> + 'static,
{
    let base_path = path("thought");

    // POST /thought
    let post_thought = warp::post()
        .and(path::end())
        .and(body::json())
        .and(with_db(db.clone()))
        .and_then(handler::thought::create::handle);

    // GET /thought/<ID>
    let get_thought = warp::get()
        .and(path!(String))
        .and(path::end())
        .and(with_db(db))
        .and_then(handler::thought::read::handle);

    base_path.and(post_thought.or(get_thought))
}

fn with_db<D>(db: Arc<D>) -> impl Filter<Extract = (Arc<D>,), Error = Infallible> + Clone
where
    D: Repo<Id = Id> + 'static,
{
    warp::any().map(move || db.clone())
}
