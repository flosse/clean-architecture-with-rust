use seed::{prelude::*, *};

// ------ ------
//     Model
// ------ ------

#[derive(Debug, Default)]
pub struct Mdl {
    pub active: bool,
    pub wait: bool,
    pub name: String,
    pub error: Option<String>,
}

// ------ ------
//    Message
// ------ ------

#[derive(Debug)]
pub enum Msg {
    Input(String),
    Add,
    Cancel,
}

// ------ ------
//    Command
// ------ ------

#[derive(Debug)]
pub enum Cmd {
    Add(String),
}

// ------ ------
//    Update
// ------ ------

pub fn update(msg: Msg, mdl: &mut Mdl) -> Option<Cmd> {
    match msg {
        Msg::Cancel => {
            mdl.active = false;
            mdl.name.clear();
            mdl.wait = false;
            mdl.error = None;
        }
        Msg::Input(txt) => {
            mdl.name = txt;
        }
        Msg::Add => {
            mdl.wait = true;
            return Some(Cmd::Add(mdl.name.clone()));
        }
    }
    None
}

// ------ ------
//     View
// ------ ------

pub fn view(mdl: &Mdl) -> Node<Msg> {
    div![
        C!["modal", IF!(mdl.active => "is-active")],
        div![C!["modal-background"]],
        div![
            C!["modal-card"],
            header![
                C!["modal-card-head"],
                p![C!["modal-card-title"], "Add new area of life"],
                button![ev(Ev::Click, |_| Msg::Cancel), C!["delete"]]
            ],
            section![
                C!["modal-card-body"],
                div![
                    C!["field"],
                    div![
                        C!["control", IF!(mdl.error.is_some() => "has-icons-right")],
                        input![
                            C!["input", IF!(mdl.error.is_some() => "is-danger")],
                            input_ev(Ev::Input, Msg::Input),
                            attrs! {
                                At::Value => mdl.name;
                                At::ReadOnly => mdl.wait.as_at_value();
                                At::Disabled => mdl.wait.as_at_value();
                                At::Placeholder => "Name of the area of life";
                            },
                        ],
                        if mdl.error.is_some() {
                            span![
                                C!["icon", "is-small", "is-right"],
                                i![C!["fas", "fa-exclamation-triangle"]]
                            ]
                        } else {
                            empty!()
                        }
                    ],
                    if let Some(err) = &mdl.error {
                        p![C!["help", "is-danger"], err]
                    } else {
                        empty!()
                    }
                ]
            ],
            footer![
                C!["modal-card-foot"],
                button![
                    ev(Ev::Click, |_| Msg::Add),
                    attrs! { At::Disabled => mdl.name.is_empty().as_at_value(); },
                    C!["button", "is-success", IF!(mdl.wait => "is-loading")],
                    "Add"
                ],
                button![ev(Ev::Click, |_| Msg::Cancel), C!["button"], "Cancel"]
            ]
        ],
    ]
}
