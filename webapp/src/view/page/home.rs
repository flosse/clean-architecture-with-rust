use crate::{domain::*, view::new_area_of_life_dialog as new_aol_dialog};
use seed::{prelude::*, *};

const CLEAN_ARCH_BLOG_URL: &str =
    "https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html";

// ------ ------
//     Model
// ------ ------

#[derive(Debug, Default)]
pub struct Mdl {
    thoughts: Vec<Thought>,
    areas_of_life: Vec<AreaOfLife>,
    input: String,
    input_error: Option<String>,
    error: Option<String>,
    wait_for_deletion: Option<ThoughtId>,
    wait: bool,
    new_aol_dialog: new_aol_dialog::Mdl,
    current_aol: Option<AreaOfLifeId>,
}

// ------ ------
//    Message
// ------ ------

#[derive(Debug)]
pub enum Msg {
    InputChanged(String),
    DeleteRequest(ThoughtId),
    SelectAreaOfLife(AreaOfLifeId),
    DeselectAreaOfLife,
    DeleteAreaOfLife(AreaOfLifeId),
    CreateRequest,
    CreateThoughtResult(Result<ThoughtId, String>),
    CreateAreaOfLifeResult(Result<AreaOfLifeId, String>),
    FindThoughtResult(Result<Thought, String>),
    FetchAllThoughtsResult(Result<Vec<Thought>, String>),
    FetchAllAreasOfLifeResult(Result<Vec<AreaOfLife>, String>),
    DeleteThoughtResult(Result<ThoughtId, String>),
    DeleteAreaOfLifeResult(Result<AreaOfLifeId, String>),
    ShowNewAreaOfLifeDialog,
    NewAOLDialog(new_aol_dialog::Msg),
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

// ------ ------
//    Update
// ------ ------

pub fn update(msg: Msg, mdl: &mut Mdl) -> Option<Cmd> {
    match msg {
        Msg::InputChanged(s) => {
            mdl.input_error = None;
            mdl.input = s;
        }
        Msg::CreateRequest => {
            if !mdl.input.is_empty() {
                let cmd = Cmd::CreateThought(mdl.input.clone(), mdl.current_aol.clone());
                mdl.wait = true;
                return Some(cmd);
            }
        }
        Msg::DeleteRequest(id) => {
            let cmd = Cmd::DeleteThought(id);
            return Some(cmd);
        }
        Msg::SelectAreaOfLife(id) => {
            mdl.current_aol = Some(id);
        }
        Msg::DeselectAreaOfLife => {
            mdl.current_aol = None;
        }
        Msg::DeleteAreaOfLife(id) => {
            let cmd = Cmd::DeleteAreaOfLife(id);
            return Some(cmd);
        }
        Msg::FindThoughtResult(Err(err)) => {
            mdl.error = Some(err);
        }
        Msg::FindThoughtResult(Ok(thought)) => {
            mdl.thoughts.push(thought);
        }
        Msg::CreateThoughtResult(res) => {
            mdl.wait = false;
            match res {
                Ok(_) => {
                    mdl.input.clear();
                    mdl.error = None;
                }
                Err(err) => {
                    mdl.input_error = Some(err.clone());
                    mdl.error = Some(err);
                }
            }
        }
        Msg::CreateAreaOfLifeResult(res) => {
            mdl.new_aol_dialog.wait = false;
            match res {
                Ok(_) => {
                    new_aol_dialog::update(new_aol_dialog::Msg::Cancel, &mut mdl.new_aol_dialog);
                }
                Err(err) => {
                    mdl.new_aol_dialog.error = Some(err);
                }
            }
        }
        Msg::FetchAllThoughtsResult(res) => match res {
            Ok(thoughts) => {
                mdl.thoughts = thoughts;
            }
            Err(err) => {
                mdl.error = Some(err);
            }
        },
        Msg::FetchAllAreasOfLifeResult(res) => match res {
            Ok(areas_of_life) => {
                mdl.areas_of_life = areas_of_life;
            }
            Err(err) => {
                mdl.error = Some(err);
            }
        },
        Msg::DeleteThoughtResult(res) => match res {
            Ok(id) => {
                mdl.thoughts.retain(|t| t.id != id);
            }
            Err(err) => {
                mdl.error = Some(err);
            }
        },
        Msg::DeleteAreaOfLifeResult(res) => match res {
            Ok(id) => {
                mdl.areas_of_life.retain(|a| a.id != id);
            }
            Err(err) => {
                mdl.error = Some(err);
            }
        },
        Msg::ShowNewAreaOfLifeDialog => {
            mdl.new_aol_dialog.active = true;
        }
        Msg::NewAOLDialog(msg) => {
            if let Some(cmd) = new_aol_dialog::update(msg, &mut mdl.new_aol_dialog) {
                match cmd {
                    new_aol_dialog::Cmd::Add(name) => {
                        return Some(Cmd::CreateAreaOfLife(name));
                    }
                }
            }
        }
    }
    None
}

// ------ ------
//     View
// ------ ------

pub fn view(mdl: &Mdl) -> Node<Msg> {
    div![
        header(),
        div![
            main_sidebar(mdl),
            main(mdl),
            new_aol_dialog::view(&mdl.new_aol_dialog).map_msg(Msg::NewAOLDialog)
        ]
    ]
}

fn main(mdl: &Mdl) -> Node<Msg> {
    main![
        id!["main"],
        error_message(mdl),
        section![
            C!["section"],
            div![
                C!["container"],
                header(),
                new_thought_input(mdl),
                thoughts_list(&mdl.thoughts, &mdl.wait_for_deletion, &mdl.current_aol)
            ]
        ],
    ]
}

fn error_message<M>(mdl: &Mdl) -> Node<M> {
    if let Some(err) = &mdl.error {
        div![
            C!["error-message"],
            style! {
              St::AlignItems => "center";
              St::Display => "flex";
              St::JustifyContent => "center";
            },
            p![err]
        ]
    } else {
        empty![]
    }
}

fn main_sidebar(mdl: &Mdl) -> Node<Msg> {
    aside![
        id!["main-sidebar"],
        nav![
            C!["menu"],
            p![
                C!["menu-label"],
                "Areas of Life",
                button![
                    ev(Ev::Click, |_| Msg::ShowNewAreaOfLifeDialog),
                    C!["button"],
                    span![
                        C!["icon", "is-right", "is-small"],
                        i![C!["fas", "fa-plus-circle"]]
                    ]
                ]
            ],
            aol_list(mdl)
        ],
    ]
}

fn aol_list(mdl: &Mdl) -> Node<Msg> {
    if mdl.areas_of_life.is_empty() {
        p![
            style! {St::Color => "#bbb"; St::FontSize => em(0.75);},
            "Currently there are no areas of life"
        ]
    } else {
        ul![
            C!["menu-list", "aol"],
            li![
                C![IF!( mdl.current_aol.is_none() => "active")],
                ev(Ev::Click, |_| Msg::DeselectAreaOfLife),
                "All"
            ],
            mdl.areas_of_life.iter().map(|aol| {
                let sel_id = aol.id.clone();
                let del_id = aol.id.clone();
                let active = mdl.current_aol.as_ref() == Some(&aol.id);
                li![
                    C![IF!( active => "active")],
                    ev(Ev::Click, |_| Msg::SelectAreaOfLife(sel_id)),
                    &aol.name,
                    button![
                        C!["button"],
                        ev(Ev::Click, |_| Msg::DeleteAreaOfLife(del_id)),
                        span![
                            C!["icon", "is-right", "is-small"],
                            i![C!["fas", "fa-minus-circle"]]
                        ]
                    ]
                ]
            }),
        ]
    }
}

fn header<M>() -> Node<M> {
    nav![
        id!["main-navbar"],
        C!["navbar"],
        div![
            C!["navbar-start"],
            div![
                C!["navbar-item"],
                h1![
                    C!["title", "is-5"],
                    "Full-Stack Clean Architecture with Rust"
                ],
                p![
                    C!["subtitle", "is-6"],
                    "An example implementation of a ",
                    a![
                        attrs! { At::Href => CLEAN_ARCH_BLOG_URL; },
                        "Clean Architecture"
                    ],
                    " written in ",
                    a![attrs! { At::Href => "https://rust-lang.org"; }, "Rust"],
                    "."
                ],
            ]
        ]
    ]
}

fn new_thought_input(mdl: &Mdl) -> Node<Msg> {
    div![
        C!["block"],
        h3![C!["title", "is-4"], "Add new thought"],
        div![
            C!["field"],
            div![
                C![
                    "control",
                    IF!(mdl.input_error.is_some() => "has-icons-right")
                ],
                input![
                    C!["input", IF!(mdl.input_error.is_some() => "is-danger")],
                    input_ev(Ev::Input, Msg::InputChanged),
                    keyboard_ev(Ev::KeyDown, |ev| {
                        if ev.key() == "Enter" {
                            Some(Msg::CreateRequest)
                        } else {
                            None
                        }
                    }),
                    attrs! {
                        At::ReadOnly => mdl.wait.as_at_value();
                        At::Disabled => mdl.wait.as_at_value();
                        At::Value => mdl.input;
                        At::Placeholder => "create a new thought";
                    },
                ],
                if mdl.input_error.is_some() {
                    span![
                        C!["icon", "is-small", "is-right"],
                        i![C!["fas", "fa-exclamation-triangle"]]
                    ]
                } else {
                    empty!()
                }
            ],
            if let Some(err) = &mdl.input_error {
                p![C!["help", "is-danger"], err]
            } else {
                empty!()
            }
        ],
    ]
}

fn thoughts_list(
    thoughts: &[Thought],
    wait_for_deletion: &Option<ThoughtId>,
    aol: &Option<AreaOfLifeId>,
) -> Node<Msg> {
    div![
        C!["block", "thoughts"],
        h3![C!["title", "is-4"], "Thoughts"],
        if thoughts.is_empty() {
            p!["Currenty there are no thoughts."]
        } else {
            ul![thoughts
                .iter()
                .filter(|t| {
                    if let Some(aol) = aol {
                        t.areas_of_life.iter().any(|x| x == aol)
                    } else {
                        true
                    }
                })
                .map(|t| {
                    let wait = wait_for_deletion
                        .as_ref()
                        .map(|id| id == &t.id)
                        .unwrap_or(false);
                    li![thought(t, wait)]
                })]
        }
    ]
}

fn thought(t: &Thought, wait_for_deletion: bool) -> Node<Msg> {
    let id = t.id.clone();
    div![
        C!["level"],
        div![C!["level-left"], div![C!["level-item"], &t.title]],
        div![
            C!["level-right"],
            div![
                C!["level-item"],
                button![
                    ev(Ev::Click, |_| Msg::DeleteRequest(id)),
                    C!["button", "is-danger"],
                    if wait_for_deletion {
                        C!["is-loading"]
                    } else {
                        C![]
                    },
                    span![C!["icon"], i![C!["fa", "fa-trash-alt"],]],
                    span!["delete"]
                ]
            ]
        ]
    ]
}
