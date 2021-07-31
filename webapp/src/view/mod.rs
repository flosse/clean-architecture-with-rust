use crate::domain::*;
use seed::prelude::*;

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
    FindThoughtResult(Result<Thought>),
}

// ------ ------
//    Command
// ------ ------

#[derive(Debug)]
pub enum Cmd {
    CreateThought(String),
}

impl From<page::Cmd> for Cmd {
    fn from(cmd: page::Cmd) -> Self {
        match cmd {
            page::Cmd::CreateThought(title) => Self::CreateThought(title),
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
        Msg::FindThoughtResult(res) => page::Msg::Home(page::home::Msg::FindThoughtResult(res)),
    };
    page::update(page_msg, &mut mdl.page).map(Cmd::from)
}

// ------ ------
//     View
// ------ ------

pub fn view(mdl: &Mdl) -> Node<Msg> {
    page::view(&mdl.page).map_msg(Msg::Page)
}
