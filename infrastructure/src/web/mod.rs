use adapter::model::app::thought::Id;
use application::gateway::repository::thought::{NewId, Repo};
use std::{net::SocketAddr, sync::Arc};
use warp::Filter;

mod handler;
mod route;
#[cfg(test)]
mod tests;
mod webapp;

pub async fn run<D>(db: Arc<D>, addr: SocketAddr)
where
    D: Repo<Id = Id> + 'static + NewId<Id>,
{
    let api = route::api(db);
    let routes = api.or(webapp::get_index()).or(webapp::get_assets());
    warp::serve(routes).run(addr).await;
}
