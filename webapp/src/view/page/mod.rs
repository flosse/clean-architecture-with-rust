use crate::domain::*;
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
    CreateThought(String),
    CreateAreaOfLife(String),
    DeleteThought(ThoughtId),
    DeleteAreaOfLife(AreaOfLifeId),
}

impl From<home::Cmd> for Cmd {
    fn from(cmd: home::Cmd) -> Self {
        match cmd {
            home::Cmd::CreateThought(title) => Self::CreateThought(title),
            home::Cmd::CreateAreaOfLife(name) => Self::CreateAreaOfLife(name),
            home::Cmd::DeleteThought(id) => Self::DeleteThought(id),
            home::Cmd::DeleteAreaOfLife(id) => Self::DeleteAreaOfLife(id),
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

pub fn view(mdl: &Mdl) -> Node<Msg> {
    match mdl {
        Mdl::Home(mdl) => home::view(mdl).map_msg(Msg::Home),
    }
}
