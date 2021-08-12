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

#[derive(Debug)]
pub enum Msg {
    View(view::Msg),
    CreateThoughtResult(Result<ThoughtId>),
    FetchAllThoughtsResult(Result<Vec<Thought>>),
    FindThoughtResult(Result<Thought>),
    DeleteThoughtResult(Result<ThoughtId>),
}

// ------ ------
//    Update
// ------ ------

fn update(msg: Msg, mdl: &mut Mdl, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::View(msg) => {
            if let Some(cmd) = view::update(msg, &mut mdl.view) {
                match cmd {
                    view::Cmd::CreateThought(title) => {
                        orders.perform_cmd(create_thought(title));
                    }
                    view::Cmd::DeleteThought(id) => {
                        orders.perform_cmd(delete_thought(id));
                    }
                }
            }
        }
        Msg::CreateThoughtResult(res) => {
            if let Ok(id) = &res {
                orders.perform_cmd(find_thought_by_id(id.clone()));
            }
            let msg = view::Msg::CreateThoughtResult(res);
            view::update(msg, &mut mdl.view);
        }
        Msg::FindThoughtResult(res) => {
            let msg = view::Msg::FindThoughtResult(res);
            view::update(msg, &mut mdl.view);
        }
        Msg::FetchAllThoughtsResult(res) => {
            let msg = view::Msg::FetchAllThoughtsResult(res);
            view::update(msg, &mut mdl.view);
        }
        Msg::DeleteThoughtResult(res) => {
            let msg = view::Msg::DeleteThoughtResult(res);
            view::update(msg, &mut mdl.view);
        }
    }
}

// -- Map usecases to messages -- //

async fn create_thought(title: String) -> Msg {
    let res = usecase::thought::create(title).await;
    Msg::CreateThoughtResult(res)
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

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Mdl {
    orders.perform_cmd(fetch_all_thoughts());
    Mdl::default()
}

// ------ ------
//     View
// ------ ------

fn view(mdl: &Mdl) -> Node<Msg> {
    view::view(&mdl.view).map_msg(Msg::View)
}

// ------ ------
//     Start
// ------ ------

fn main() {
    App::start("app", init, update, view);
}
