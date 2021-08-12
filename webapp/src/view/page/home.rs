use crate::domain::*;
use seed::{prelude::*, *};

const CLEAN_ARCH_BLOG_URL: &str =
    "https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html";

// ------ ------
//     Model
// ------ ------

#[derive(Debug, Default)]
pub struct Mdl {
    thoughts: Vec<Thought>,
    input: String,
    input_error: Option<String>,
    error: Option<String>,
    wait_for_deletion: Option<ThoughtId>,
    wait: bool,
}

// ------ ------
//    Message
// ------ ------

#[derive(Debug)]
pub enum Msg {
    InputChanged(String),
    DeleteRequest(ThoughtId),
    CreateRequest,
    CreateThoughtResult(Result<ThoughtId, String>),
    FindThoughtResult(Result<Thought, String>),
    FetchAllThoughtsResult(Result<Vec<Thought>, String>),
    DeleteThoughtResult(Result<ThoughtId, String>),
}

// ------ ------
//    Command
// ------ ------

#[derive(Debug)]
pub enum Cmd {
    CreateThought(String),
    DeleteThought(ThoughtId),
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
                let cmd = Cmd::CreateThought(mdl.input.clone());
                mdl.wait = true;
                return Some(cmd);
            }
        }
        Msg::DeleteRequest(id) => {
            let cmd = Cmd::DeleteThought(id);
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
        Msg::FetchAllThoughtsResult(res) => match res {
            Ok(thoughts) => {
                mdl.thoughts = thoughts;
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
    }
    None
}

// ------ ------
//     View
// ------ ------

pub fn view(mdl: &Mdl) -> Node<Msg> {
    div![
        if let Some(err) = &mdl.error {
            div![
                style! {
                  St::AlignItems => "center";
                  St::Display => "flex";
                  St::JustifyContent => "center";
                  St::Padding =>  em(0.5);
                  St::FontSize =>  rem(0.875);
                  St::Color => "#f14668";
                  St::BackgroundColor => "#fee";
                },
                p![err]
            ]
        } else {
            empty![]
        },
        section![
            C!["section"],
            div![
                C!["container"],
                header(),
                new_thought_input(mdl),
                thoughts_list(&mdl.thoughts, &mdl.wait_for_deletion)
            ]
        ]
    ]
}

fn header<M>() -> Node<M> {
    div![
        C!["block"],
        h1![C!["title"], "Full-Stack Clean Architecture with Rust"],
        p![
            C!["subtitle"],
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
}

fn new_thought_input(mdl: &Mdl) -> Node<Msg> {
    div![
        C!["block"],
        h3![C!["title", "is-4"], "Add new thought"],
        div![
            C!["field"],
            div![
                if mdl.input_error.is_some() {
                    C!["control", "has-icons-right"]
                } else {
                    C!["control"]
                },
                input![
                    if mdl.input_error.is_some() {
                        C!["input", "is-danger"]
                    } else {
                        C!["input"]
                    },
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

fn thoughts_list(thoughts: &[Thought], wait_for_deletion: &Option<ThoughtId>) -> Node<Msg> {
    div![
        C!["block", "thoughts"],
        h3![C!["title", "is-4"], "Thoughts"],
        if thoughts.is_empty() {
            p!["Currenty there are no thoughts."]
        } else {
            ul![thoughts.iter().map(|t| {
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
