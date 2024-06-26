use std::future::Future;

use seed::prelude::*;

use cawr_web_app_kern::{self as kern, domain, UsecaseResult};

mod view;

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

pub type Msg = kern::Msg<view::Msg>;

impl From<view::Msg> for Msg {
    fn from(from: view::Msg) -> Self {
        Msg::View(from)
    }
}

// ------ ------
//    Update
// ------ ------

fn update(msg: Msg, mdl: &mut Mdl, orders: &mut impl Orders<Msg>) {
    log::debug!("{msg:?}");
    match msg {
        Msg::View(msg) => {
            if let Some(cmd) = view::update(msg, &mut mdl.view) {
                match cmd {
                    view::Cmd::CreateThought(title, areas_of_life) => {
                        run_usecase(orders, kern::create_thought(title, areas_of_life));
                    }
                    view::Cmd::UpdateThought(thought) => {
                        run_usecase(orders, kern::update_thought(thought));
                    }
                    view::Cmd::CreateAreaOfLife(name) => {
                        run_usecase(orders, kern::create_area_of_life(name));
                    }
                    view::Cmd::DeleteThought(id) => {
                        run_usecase(orders, kern::delete_thought(id));
                    }
                    view::Cmd::DeleteAreaOfLife(id) => {
                        run_usecase(orders, kern::delete_area_of_life(id));
                    }
                    view::Cmd::UpdateAreaOfLife(aol) => {
                        run_usecase(orders, kern::update_area_of_life(aol));
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
        Msg::Usecase(msg) => match msg {
            UsecaseResult::CreateThought(res) => {
                if let Ok(id) = &res {
                    run_usecase(orders, kern::find_thought_by_id(*id));
                }
                let msg = view::Msg::CreateThoughtResult(res);
                orders.send_msg(msg.into());
            }
            UsecaseResult::UpdateThought(id, res) => {
                // Re-fetch the thought
                run_usecase(orders, kern::find_thought_by_id(id));
                let msg = view::Msg::UpdateThoughtResult(res);
                orders.send_msg(msg.into());
            }
            UsecaseResult::CreateAreaOfLife(res) => {
                if res.is_ok() {
                    run_usecase(orders, kern::fetch_all_areas_of_life());
                }
                let msg = view::Msg::CreateAreaOfLifeResult(res);
                orders.send_msg(msg.into());
            }
            UsecaseResult::UpdateAreaOfLife(_id, res) => {
                // Re-fetch
                run_usecase(orders, kern::fetch_all_areas_of_life());
                let msg = view::Msg::UpdateAreaOfLifeResult(res);
                orders.send_msg(msg.into());
            }
            UsecaseResult::FindThought(res) => {
                let msg = view::Msg::FindThoughtResult(res);
                orders.send_msg(msg.into());
            }
            UsecaseResult::FetchAllThoughts(res) => {
                let msg = view::Msg::FetchAllThoughtsResult(res);
                orders.send_msg(msg.into());
            }
            UsecaseResult::FetchAllAreasOfLife(res) => {
                let msg = view::Msg::FetchAllAreasOfLifeResult(res);
                orders.send_msg(msg.into());
            }
            UsecaseResult::DeleteThought(res) => {
                let msg = view::Msg::DeleteThoughtResult(res);
                orders.send_msg(msg.into());
            }
            UsecaseResult::DeleteAreaOfLife(res) => {
                let msg = view::Msg::DeleteAreaOfLifeResult(res);
                orders.send_msg(msg.into());
            }
        },
    }
}

fn run_usecase<F, O>(orders: &mut O, usecase: F)
where
    F: Future<Output = UsecaseResult> + 'static,
    O: Orders<Msg>,
{
    orders.perform_cmd(async {
        let result = usecase.await;
        Msg::Usecase(result)
    });
}

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Mdl {
    run_usecase(orders, kern::fetch_all_thoughts());
    run_usecase(orders, kern::fetch_all_areas_of_life());
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

pub fn start(mount: web_sys::Element) {
    App::start(mount, init, update, view);
}
