use crate::domain::*;
use seed::prelude::*;

pub mod new_area_of_life_dialog;
pub mod page;

// ------ ------
//     Model
// ------ ------

#[derive(Debug, Default)]
pub struct Mdl {
    page: page::Mdl,
}

// ------ ------
//    Message
// ------ ------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Msg {
    Page(page::Msg),
    CreateThoughtResult(Result<ThoughtId>),
    CreateAreaOfLifeResult(Result<AreaOfLifeId>),
    FindThoughtResult(Result<Thought>),
    FetchAllThoughtsResult(Result<Vec<Thought>>),
    FetchAllAreasOfLifeResult(Result<Vec<AreaOfLife>>),
    DeleteThoughtResult(Result<ThoughtId>),
    DeleteAreaOfLifeResult(Result<AreaOfLifeId>),
}

// ------ ------
//    Command
// ------ ------

#[derive(Debug)]
pub enum Cmd {
    CreateThought(String, Option<AreaOfLifeId>),
    CreateAreaOfLife(String),
    DeleteThought(ThoughtId),
    DeleteAreaOfLife(AreaOfLifeId),
}

impl From<page::Cmd> for Cmd {
    fn from(cmd: page::Cmd) -> Self {
        match cmd {
            page::Cmd::CreateThought(title, area_of_life) => {
                Self::CreateThought(title, area_of_life)
            }
            page::Cmd::CreateAreaOfLife(name) => Self::CreateAreaOfLife(name),
            page::Cmd::DeleteThought(id) => Self::DeleteThought(id),
            page::Cmd::DeleteAreaOfLife(id) => Self::DeleteAreaOfLife(id),
        }
    }
}

// ------ ------
//    Update
// ------ ------

pub fn update(msg: Msg, mdl: &mut Mdl) -> Option<Cmd> {
    let page_msg = match msg {
        Msg::Page(msg) => msg,
        Msg::CreateThoughtResult(res) => page::Msg::Home(page::home::Msg::CreateThoughtResult(res)),
        Msg::CreateAreaOfLifeResult(res) => {
            page::Msg::Home(page::home::Msg::CreateAreaOfLifeResult(res))
        }
        Msg::FindThoughtResult(res) => page::Msg::Home(page::home::Msg::FindThoughtResult(res)),
        Msg::DeleteThoughtResult(res) => page::Msg::Home(page::home::Msg::DeleteThoughtResult(res)),
        Msg::DeleteAreaOfLifeResult(res) => {
            page::Msg::Home(page::home::Msg::DeleteAreaOfLifeResult(res))
        }
        Msg::FetchAllThoughtsResult(res) => {
            page::Msg::Home(page::home::Msg::FetchAllThoughtsResult(res))
        }
        Msg::FetchAllAreasOfLifeResult(res) => {
            page::Msg::Home(page::home::Msg::FetchAllAreasOfLifeResult(res))
        }
    };
    page::update(page_msg, &mut mdl.page).map(Cmd::from)
}

// ------ ------
//     View
// ------ ------

pub fn view(mdl: &Mdl) -> Node<Msg> {
    page::view(&mdl.page).map_msg(Msg::Page)
}
