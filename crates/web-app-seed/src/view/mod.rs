use seed::prelude::*;

use crate::domain::*;

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
    UpdateThoughtResult(Result<()>),
    CreateAreaOfLifeResult(Result<AreaOfLifeId>),
    FindThoughtResult(Result<Thought>),
    FetchAllThoughtsResult(Result<Vec<Thought>>),
    FetchAllAreasOfLifeResult(Result<Vec<AreaOfLife>>),
    DeleteThoughtResult(Result<ThoughtId>),
    DeleteAreaOfLifeResult(Result<AreaOfLifeId>),
    UpdateAreaOfLifeResult(Result<()>),
}

// ------ ------
//    Command
// ------ ------

#[derive(Debug)]
pub enum Cmd {
    CreateThought(String, Option<AreaOfLifeId>),
    UpdateThought(Thought),
    DeleteThought(ThoughtId),
    CreateAreaOfLife(String),
    DeleteAreaOfLife(AreaOfLifeId),
    UpdateAreaOfLife(AreaOfLife),
    SendMessages(Vec<Msg>),
}

impl From<page::Cmd> for Cmd {
    fn from(cmd: page::Cmd) -> Self {
        use page::Cmd as C;
        match cmd {
            C::CreateThought(title, aol) => Self::CreateThought(title, aol),
            C::UpdateThought(thought) => Self::UpdateThought(thought),
            C::DeleteThought(id) => Self::DeleteThought(id),
            C::CreateAreaOfLife(name) => Self::CreateAreaOfLife(name),
            C::DeleteAreaOfLife(id) => Self::DeleteAreaOfLife(id),
            C::UpdateAreaOfLife(aol) => Self::UpdateAreaOfLife(aol),
            C::SendMessages(m) => Self::SendMessages(m.into_iter().map(Msg::Page).collect()),
        }
    }
}

// ------ ------
//    Update
// ------ ------

#[must_use]
pub fn update(msg: Msg, mdl: &mut Mdl) -> Option<Cmd> {
    let page_msg = match msg {
        Msg::Page(msg) => msg,
        Msg::CreateThoughtResult(res) => page::Msg::Home(page::home::Msg::CreateThoughtResult(res)),
        Msg::UpdateThoughtResult(res) => page::Msg::Home(page::home::Msg::UpdateThoughtResult(res)),
        Msg::CreateAreaOfLifeResult(res) => {
            page::Msg::Home(page::home::Msg::CreateAreaOfLifeResult(res))
        }
        Msg::FindThoughtResult(res) => page::Msg::Home(page::home::Msg::FindThoughtResult(res)),
        Msg::DeleteThoughtResult(res) => page::Msg::Home(page::home::Msg::DeleteThoughtResult(res)),
        Msg::DeleteAreaOfLifeResult(res) => {
            page::Msg::Home(page::home::Msg::DeleteAreaOfLifeResult(res))
        }
        Msg::UpdateAreaOfLifeResult(res) => {
            page::Msg::Home(page::home::Msg::UpdateAreaOfLifeResult(res))
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

pub fn view(mdl: &Mdl) -> Vec<Node<Msg>> {
    page::view(&mdl.page).map_msg(Msg::Page)
}
