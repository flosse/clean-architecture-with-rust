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
}

impl From<home::Cmd> for Cmd {
    fn from(cmd: home::Cmd) -> Self {
        let home::Cmd::CreateThought(title) = cmd;
        Self::CreateThought(title)
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
