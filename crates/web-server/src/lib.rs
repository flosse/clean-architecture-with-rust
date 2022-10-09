use cawr_adapter::{api::Api, db::Db, presenter::http_json_api::Presenter};
use std::{net::SocketAddr, sync::Arc};
use warp::Filter;

mod handler;
mod route;
#[cfg(test)]
mod tests;
mod webapp;

type AppApi<D> = Api<D, Presenter>;

pub async fn run<D>(db: Arc<D>, addr: SocketAddr)
where
    D: Db,
{
    let web_app_api = Api::new(db, Presenter::default());
    let api = route::api(web_app_api);
    let routes = api.or(webapp::get_index()).or(webapp::get_assets());
    warp::serve(routes).run(addr).await;
}
