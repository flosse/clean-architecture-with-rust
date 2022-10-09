use crate::handler;
use cawr_adapter::{controller, db::Db, presenter::http_json_api::Presenter};
use std::{convert::Infallible, sync::Arc};
use warp::{body, path, Filter, Rejection, Reply};

pub fn api<D>(db: Arc<D>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    D: Db,
{
    let presenter = Presenter::default();
    let db = Arc::clone(&db);
    let thought_controller = Arc::new(controller::thought::Controller::new(
        db.clone(),
        presenter.clone(),
    ));
    let area_of_life_controller =
        Arc::new(controller::area_of_life::Controller::new(db, presenter));

    // POST /api/thought
    let post_thought = warp::post()
        .and(path::end())
        .and(body::json())
        .and(with_controller(thought_controller.clone()))
        .and_then(handler::thought::create::handle);

    // PUT /api/thought/<ID>
    let put_thought = warp::put()
        .and(path!(String))
        .and(path::end())
        .and(body::json())
        .and(with_controller(thought_controller.clone()))
        .and_then(handler::thought::update::handle);

    // GET /api/thought
    let get_thoughts = warp::get()
        .and(path::end())
        .and(with_controller(thought_controller.clone()))
        .and_then(handler::thought::read_all::handle);

    // GET /api/thought/<ID>
    let get_thought = warp::get()
        .and(path!(String))
        .and(path::end())
        .and(with_controller(thought_controller.clone()))
        .and_then(handler::thought::find_by_id::handle);

    // DELETE /api/thought/<ID>
    let delete_thought = warp::delete()
        .and(path!(String))
        .and(path::end())
        .and(with_controller(thought_controller))
        .and_then(handler::thought::delete::handle);

    // POST /api/area-of-life
    let post_area_of_life = warp::post()
        .and(path::end())
        .and(body::json())
        .and(with_controller(area_of_life_controller.clone()))
        .and_then(handler::area_of_life::create::handle);

    // PUT /api/area-of-life/<ID>
    let put_area_of_life = warp::put()
        .and(path!(String))
        .and(path::end())
        .and(body::json())
        .and(with_controller(area_of_life_controller.clone()))
        .and_then(handler::area_of_life::update::handle);

    // GET /api/area-of-life
    let get_areas_of_life = warp::get()
        .and(path::end())
        .and(with_controller(area_of_life_controller.clone()))
        .and_then(handler::area_of_life::read_all::handle);

    // DELETE /api/area-of-life/<ID>
    let delete_area_of_life = warp::delete()
        .and(path!(String))
        .and(path::end())
        .and(with_controller(area_of_life_controller))
        .and_then(handler::area_of_life::delete::handle);

    let base_path = path("api");
    let thought = path("thought").and(
        post_thought
            .or(put_thought)
            .or(get_thoughts)
            .or(get_thought)
            .or(delete_thought),
    );
    let area_of_life = path("area-of-life").and(
        post_area_of_life
            .or(put_area_of_life)
            .or(get_areas_of_life)
            .or(delete_area_of_life),
    );
    base_path.and(thought.or(area_of_life))
}

fn with_controller<C>(
    controller: Arc<C>,
) -> impl Filter<Extract = (Arc<C>,), Error = Infallible> + Clone
where
    C: Send + Sync,
{
    warp::any().map(move || controller.clone())
}
