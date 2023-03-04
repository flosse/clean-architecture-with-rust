use seed::prelude::*;

mod api;
mod domain;
mod usecase;
mod view;

use domain::*;

// ------ ------
//     Model
// ------ ------

#[derive(Debug, Default)]
pub struct Mdl {
    view: view::Mdl,
}

// ------ ------
//    Message
// ------ ------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

#[derive(derive_more::From, Debug)]
pub enum Msg {
    #[from]
    View(view::Msg),
    CreateThoughtResult(Result<ThoughtId>),
    UpdateThoughtResult(ThoughtId, Result<()>),
    CreateAreaOfLifeResult(Result<AreaOfLifeId>),
    UpdateAreaOfLifeResult(AreaOfLifeId, Result<()>),
    FetchAllThoughtsResult(Result<Vec<Thought>>),
    FetchAllAreasOfLifeResult(Result<Vec<AreaOfLife>>),
    FindThoughtResult(Result<Thought>),
    DeleteThoughtResult(Result<ThoughtId>),
    DeleteAreaOfLifeResult(Result<AreaOfLifeId>),
}

// ------ ------
//    Update
// ------ ------

fn update(msg: Msg, mdl: &mut Mdl, orders: &mut impl Orders<Msg>) {
    seed::log!(msg);
    match msg {
        Msg::View(msg) => {
            if let Some(cmd) = view::update(msg, &mut mdl.view) {
                match cmd {
                    view::Cmd::CreateThought(title, areas_of_life) => {
                        orders.perform_cmd(create_thought(title, areas_of_life));
                    }
                    view::Cmd::UpdateThought(thought) => {
                        orders.perform_cmd(update_thought(thought));
                    }
                    view::Cmd::CreateAreaOfLife(name) => {
                        orders.perform_cmd(create_area_of_life(name));
                    }
                    view::Cmd::DeleteThought(id) => {
                        orders.perform_cmd(delete_thought(id));
                    }
                    view::Cmd::DeleteAreaOfLife(id) => {
                        orders.perform_cmd(delete_area_of_life(id));
                    }
                    view::Cmd::UpdateAreaOfLife(aol) => {
                        orders.perform_cmd(update_area_of_life(aol));
                    }
                    view::Cmd::SendMessages(messages) => {
                        orders.skip();
                        for m in messages {
                            orders.send_msg(Msg::View(m));
                        }
                    }
                }
            }
        }
        Msg::CreateThoughtResult(res) => {
            if let Ok(id) = &res {
                orders.perform_cmd(find_thought_by_id(*id));
            }
            let msg = view::Msg::CreateThoughtResult(res);
            orders.send_msg(msg.into());
        }
        Msg::UpdateThoughtResult(id, res) => {
            // Re-fetch the thought
            orders.perform_cmd(find_thought_by_id(id));
            let msg = view::Msg::UpdateThoughtResult(res);
            orders.send_msg(msg.into());
        }
        Msg::CreateAreaOfLifeResult(res) => {
            if res.is_ok() {
                orders.perform_cmd(fetch_all_areas_of_life());
            }
            let msg = view::Msg::CreateAreaOfLifeResult(res);
            orders.send_msg(msg.into());
        }
        Msg::UpdateAreaOfLifeResult(_id, res) => {
            // Re-fetch
            orders.perform_cmd(fetch_all_areas_of_life());
            let msg = view::Msg::UpdateAreaOfLifeResult(res);
            orders.send_msg(msg.into());
        }
        Msg::FindThoughtResult(res) => {
            let msg = view::Msg::FindThoughtResult(res);
            orders.send_msg(msg.into());
        }
        Msg::FetchAllThoughtsResult(res) => {
            let msg = view::Msg::FetchAllThoughtsResult(res);
            orders.send_msg(msg.into());
        }
        Msg::FetchAllAreasOfLifeResult(res) => {
            let msg = view::Msg::FetchAllAreasOfLifeResult(res);
            orders.send_msg(msg.into());
        }
        Msg::DeleteThoughtResult(res) => {
            let msg = view::Msg::DeleteThoughtResult(res);
            orders.send_msg(msg.into());
        }
        Msg::DeleteAreaOfLifeResult(res) => {
            let msg = view::Msg::DeleteAreaOfLifeResult(res);
            orders.send_msg(msg.into());
        }
    }
}

// -- Map usecases to messages -- //

async fn create_thought(title: String, area_of_life: Option<AreaOfLifeId>) -> Msg {
    let areas_of_life = area_of_life.map(|id| vec![id]).unwrap_or_default();
    let res = usecase::thought::create(title, areas_of_life).await;
    Msg::CreateThoughtResult(res)
}

async fn update_thought(thought: Thought) -> Msg {
    let id = thought.id;
    let res = usecase::thought::update(thought).await;
    Msg::UpdateThoughtResult(id, res)
}

async fn fetch_all_thoughts() -> Msg {
    let res = usecase::thought::fetch_all().await;
    Msg::FetchAllThoughtsResult(res)
}

async fn find_thought_by_id(id: domain::ThoughtId) -> Msg {
    let res = usecase::thought::find_by_id(&id).await;
    Msg::FindThoughtResult(res)
}

async fn delete_thought(id: domain::ThoughtId) -> Msg {
    let res = usecase::thought::delete(&id).await;
    Msg::DeleteThoughtResult(res.map(|_| id))
}

async fn create_area_of_life(name: String) -> Msg {
    let res = usecase::area_of_life::create(name).await;
    Msg::CreateAreaOfLifeResult(res)
}

async fn update_area_of_life(aol: AreaOfLife) -> Msg {
    let id = aol.id;
    let res = usecase::area_of_life::update(aol).await;
    Msg::UpdateAreaOfLifeResult(id, res)
}

async fn fetch_all_areas_of_life() -> Msg {
    let res = usecase::area_of_life::fetch_all().await;
    Msg::FetchAllAreasOfLifeResult(res)
}

async fn delete_area_of_life(id: domain::AreaOfLifeId) -> Msg {
    let res = usecase::area_of_life::delete(&id).await;
    Msg::DeleteAreaOfLifeResult(res.map(|_| id))
}

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Mdl {
    orders.perform_cmd(fetch_all_thoughts());
    orders.perform_cmd(fetch_all_areas_of_life());
    Mdl::default()
}

// ------ ------
//     View
// ------ ------

fn view(mdl: &Mdl) -> impl IntoNodes<Msg> {
    view::view(&mdl.view).map_msg(Msg::View)
}

// ------ ------
//     Start
// ------ ------

fn main() {
    App::start("app", init, update, view);
}
