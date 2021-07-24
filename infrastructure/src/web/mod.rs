use adapter::id::thought::Id;
use application::gateway::repository::thought::Repo;
use std::{net::SocketAddr, sync::Arc};

mod handler;
mod route;
#[cfg(test)]
mod tests;

pub async fn run<D>(db: Arc<D>)
where
    D: Repo<Id = Id> + 'static,
{
    let api = route::api(db);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    warp::serve(api).run(addr).await;
}
