use crate::domain::{AreaOfLife, AreaOfLifeId, Thought, ThoughtId};
use seed::prelude::*;

pub mod home;

// ------ ------
//     Model
// ------ ------

#[derive(Debug)]
pub enum Mdl {
    Home(home::Mdl),
}

impl Default for Mdl {
    fn default() -> Self {
        Self::Home(home::Mdl::default())
    }
}

// ------ ------
//    Message
// ------ ------

#[derive(Debug)]
pub enum Msg {
    Home(home::Msg),
}

// ------ ------
//    Command
// ------ ------

#[derive(Debug)]
pub enum Cmd {
    CreateThought(String, Option<AreaOfLifeId>),
    UpdateThought(Thought),
    CreateAreaOfLife(String),
    DeleteThought(ThoughtId),
    DeleteAreaOfLife(AreaOfLifeId),
    UpdateAreaOfLife(AreaOfLife),
    SendMessages(Vec<Msg>),
}

impl From<home::Cmd> for Cmd {
    fn from(cmd: home::Cmd) -> Self {
        use home::Cmd as C;
        match cmd {
            C::CreateThought(title, area_of_life) => Self::CreateThought(title, area_of_life),
            C::UpdateThought(thought) => Self::UpdateThought(thought),
            C::CreateAreaOfLife(name) => Self::CreateAreaOfLife(name),
            C::DeleteThought(id) => Self::DeleteThought(id),
            C::DeleteAreaOfLife(id) => Self::DeleteAreaOfLife(id),
            C::UpdateAreaOfLife(aol) => Self::UpdateAreaOfLife(aol),
            C::SendMessages(m) => Self::SendMessages(m.into_iter().map(Msg::Home).collect()),
        }
    }
}

// ------ ------
//    Update
// ------ ------

pub fn update(msg: Msg, mdl: &mut Mdl) -> Option<Cmd> {
    match msg {
        Msg::Home(msg) => {
            let Mdl::Home(mdl) = mdl;
            home::update(msg, mdl).map(Cmd::from)
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(mdl: &Mdl) -> Vec<Node<Msg>> {
    match mdl {
        Mdl::Home(mdl) => home::view(mdl).map_msg(Msg::Home),
    }
}
