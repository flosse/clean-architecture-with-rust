use std::collections::HashMap;

use seed::{
    a, aside, attrs, button, div, empty, h1, h3, i, id, input, li, main, nav, p, prelude::*,
    section, span, style, ul, C, IF,
};

use crate::{
    domain::{AreaOfLife, AreaOfLifeId, Thought, ThoughtId},
    view::new_area_of_life_dialog as new_aol_dialog,
};

const CLEAN_ARCH_BLOG_URL: &str =
    "https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html";

// ------ ------
//     Model
// ------ ------

#[derive(Debug, Default)]
pub struct Mdl {
    thoughts: HashMap<ThoughtId, Thought>,
    areas_of_life: Vec<AreaOfLife>,
    areas_of_life_edits: HashMap<AreaOfLifeId, AreaOfLife>,
    input: String,
    title_input: String,
    title_input_el: ElRef<web_sys::HtmlInputElement>,
    input_error: Option<String>,
    error: Option<String>,
    wait_for_deletion: Option<ThoughtId>,
    wait: bool,
    new_aol_dialog: new_aol_dialog::Mdl,
    current_thought: Option<ThoughtId>,
    current_aol: Option<AreaOfLifeId>,
    edit_areas_of_life: bool,
}

// ------ ------
//    Message
// ------ ------

type Result<T> = std::result::Result<T, String>;

#[derive(Debug)]
pub enum Msg {
    // -- Thought -- //
    InputChanged(String),
    TitleChanged(String),
    CancleTitleEdit,
    SelectRequest(ThoughtId),
    DeleteRequest(ThoughtId),
    CreateRequest,
    DeleteThoughtResult(Result<ThoughtId>),
    FindThoughtResult(Result<Thought>),
    FetchAllThoughtsResult(Result<Vec<Thought>>),
    CreateThoughtResult(Result<ThoughtId>),
    UpdateThoughtResult(Result<()>),
    // -- Area of Life -- //
    AreaOfLifeNameChanged(AreaOfLifeId, String),
    CancleAreaOfLifeNameEdit(AreaOfLifeId),
    SelectAreaOfLife(AreaOfLifeId),
    DeselectAreaOfLife,
    DeleteAreaOfLife(AreaOfLifeId),
    UpdateAreaOfLifeName(AreaOfLifeId),
    CreateAreaOfLifeResult(Result<AreaOfLifeId>),
    FetchAllAreasOfLifeResult(Result<Vec<AreaOfLife>>),
    DeleteAreaOfLifeResult(Result<AreaOfLifeId>),
    UpdateAreaOfLifeResult(Result<()>),
    ShowNewAreaOfLifeDialog,
    EditAreasOfLife(bool),
    NewAOLDialog(new_aol_dialog::Msg),
}

// ------ ------
//    Command
// ------ ------

#[derive(Debug)]
pub enum Cmd {
    // -- Thought -- //
    CreateThought(String, Option<AreaOfLifeId>),
    UpdateThought(Thought),
    DeleteThought(ThoughtId),
    // -- Area of Life -- //
    UpdateAreaOfLife(AreaOfLife),
    CreateAreaOfLife(String),
    DeleteAreaOfLife(AreaOfLifeId),
    // -- Misc -- //
    SendMessages(Vec<Msg>),
}

// ------ ------
//    Update
// ------ ------

#[must_use]
pub fn update(msg: Msg, mdl: &mut Mdl) -> Option<Cmd> {
    match msg {
        Msg::InputChanged(s) => {
            mdl.input_error = None;
            mdl.input = s;
        }
        Msg::TitleChanged(title) => {
            mdl.title_input.clone_from(&title);
            if let Some(id) = &mdl.current_thought {
                if let Some(thought) = mdl.thoughts.get_mut(id) {
                    let updated = Thought {
                        id: thought.id,
                        title,
                        areas_of_life: thought.areas_of_life.clone(),
                    };
                    let cmd = Cmd::UpdateThought(updated);
                    return Some(cmd);
                }
            }
        }
        Msg::AreaOfLifeNameChanged(id, name) => {
            if let Some(original_aol) = mdl.areas_of_life.iter_mut().find(|aol| aol.id == id) {
                mdl.areas_of_life_edits
                    .entry(id)
                    .or_insert_with(|| original_aol.clone())
                    .name = name;
            }
        }
        Msg::CancleAreaOfLifeNameEdit(id) => {
            mdl.areas_of_life_edits.remove(&id);
        }
        Msg::UpdateAreaOfLifeName(id) => {
            if let Some(aol) = mdl.areas_of_life_edits.get(&id) {
                let cmd = Cmd::UpdateAreaOfLife(aol.clone());
                return Some(cmd);
            }
        }
        Msg::CancleTitleEdit => {
            mdl.title_input = mdl
                .current_thought
                .and_then(|id| mdl.thoughts.get(&id).map(|t| t.title.clone()))
                .unwrap_or_default();
            if let Some(el) = mdl.title_input_el.get() {
                el.blur().expect("Unable to blur input element");
            }
        }
        Msg::CreateRequest => {
            if !mdl.input.is_empty() {
                let cmd = Cmd::CreateThought(mdl.input.clone(), mdl.current_aol);
                mdl.wait = true;
                return Some(cmd);
            }
        }
        Msg::SelectRequest(id) => {
            mdl.title_input = mdl
                .thoughts
                .get(&id)
                .map(|t| t.title.clone())
                .unwrap_or_default();
            mdl.current_thought = Some(id);
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
            mdl.thoughts.insert(thought.id, thought);
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
        Msg::UpdateThoughtResult(res) => {
            mdl.error = match res {
                Ok(()) => None,
                Err(err) => Some(err),
            };
        }
        Msg::CreateAreaOfLifeResult(res) => {
            mdl.new_aol_dialog.wait = false;
            match res {
                Ok(_) => {
                    let msg = Msg::NewAOLDialog(new_aol_dialog::Msg::Cancel);
                    return Some(Cmd::SendMessages(vec![msg]));
                }
                Err(err) => {
                    mdl.new_aol_dialog.error = Some(err);
                }
            }
        }
        Msg::FetchAllThoughtsResult(res) => match res {
            Ok(thoughts) => {
                mdl.thoughts = thoughts.into_iter().map(|t| (t.id, t)).collect();
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
                mdl.thoughts.remove(&id);
            }
            Err(err) => {
                mdl.error = Some(err);
            }
        },
        Msg::DeleteAreaOfLifeResult(res) => match res {
            Ok(id) => {
                if mdl.current_aol == Some(id) {
                    mdl.current_aol = None;
                }
                mdl.areas_of_life.retain(|a| a.id != id);
            }
            Err(err) => {
                mdl.error = Some(err);
            }
        },
        Msg::UpdateAreaOfLifeResult(res) => {
            if let Err(err) = res {
                mdl.error = Some(err);
            }
        }
        Msg::EditAreasOfLife(edit) => {
            if !edit {
                mdl.areas_of_life_edits.clear();
            }
            mdl.edit_areas_of_life = edit;
        }
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

pub fn view(mdl: &Mdl) -> Vec<Node<Msg>> {
    vec![
        header(),
        main_sidebar(mdl),
        main(mdl),
        edit_sidebar(mdl),
        new_aol_dialog::view(&mdl.new_aol_dialog).map_msg(Msg::NewAOLDialog),
    ]
}

fn main(mdl: &Mdl) -> Node<Msg> {
    let thoughts = mdl.thoughts.values().collect::<Vec<_>>();
    main![
        id!["main"],
        error_message(mdl),
        section![
            C!["section"],
            div![
                C!["container"],
                new_thought_input(mdl),
                thoughts_list(&thoughts, &mdl.wait_for_deletion, &mdl.current_aol)
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
    let aol_edit = mdl.edit_areas_of_life;
    aside![
        id!["main-sidebar"],
        nav![
            C!["menu"],
            p![
                C!["menu-label"],
                "Areas of Life",
                if mdl.areas_of_life.is_empty() {
                    new_area_of_life_button()
                } else {
                    edit_areas_of_life_button(aol_edit)
                }
            ],
            aol_list(mdl)
        ],
    ]
}

fn edit_sidebar(mdl: &Mdl) -> Node<Msg> {
    aside![
        id!["edit-sidebar"],
        div![
            C!["field"],
            div![
                C!["control"],
                input![
                    C!["input"],
                    el_ref(&mdl.title_input_el),
                    input_ev(Ev::Input, Msg::TitleChanged),
                    keyboard_ev(Ev::KeyUp, |ev| {
                        if ev.key() == "Escape" {
                            ev.prevent_default();
                            return Some(Msg::CancleTitleEdit);
                        }
                        None
                    }),
                    attrs! { At::Value => &mdl.title_input }
                ]
            ]
        ]
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
                let id = aol.id;
                let active = mdl.current_aol.as_ref() == Some(&id);
                let clz = C![IF!( active => "active")];

                if mdl.edit_areas_of_life {
                    let name = mdl
                        .areas_of_life_edits
                        .get(&id)
                        .map_or(&aol.name, |aol| &aol.name);
                    li![
                        clz,
                        C!["field", "has-addons"],
                        div![
                            C!["control"],
                            input![
                                C!["input"],
                                attrs! {
                                  At::Value => name;
                                },
                                input_ev(Ev::Input, move |name| Msg::AreaOfLifeNameChanged(
                                    id, name
                                )),
                                ev(Ev::Blur, move |_| Msg::UpdateAreaOfLifeName(id)),
                                keyboard_ev(Ev::KeyUp, move |ev| {
                                    match &*ev.key() {
                                        "Escape" => {
                                            ev.prevent_default();
                                            Some(Msg::CancleAreaOfLifeNameEdit(id))
                                        }
                                        "Enter" => {
                                            ev.prevent_default();
                                            Some(Msg::UpdateAreaOfLifeName(id))
                                        }
                                        _ => None,
                                    }
                                }),
                            ]
                        ],
                        div![
                            C!["control"],
                            button![
                                C!["button"],
                                ev(Ev::Click, move |_| Msg::DeleteAreaOfLife(id)),
                                span![
                                    C!["icon", "is-right", "is-small", "has-text-danger"],
                                    i![C!["fas", "fa-minus-circle"]]
                                ]
                            ]
                        ]
                    ]
                } else {
                    li![
                        clz,
                        ev(Ev::Click, move |_| Msg::SelectAreaOfLife(id)),
                        &aol.name,
                    ]
                }
            }),
            if mdl.edit_areas_of_life {
                li![new_area_of_life_button()]
            } else {
                empty!()
            },
        ]
    }
}

fn new_area_of_life_button() -> Node<Msg> {
    button![
        ev(Ev::Click, move |_| Msg::ShowNewAreaOfLifeDialog),
        C!["button"],
        style! { St::Float => "right"; },
        span![
            C!["icon", "is-right", "is-small", "has-text-success"],
            i![C!["fas", "fa-plus"]]
        ]
    ]
}

fn edit_areas_of_life_button(aol_edit: bool) -> Node<Msg> {
    button![
        ev(Ev::Click, move |_| Msg::EditAreasOfLife(!aol_edit)),
        C!["button"],
        span![
            C![
                "icon",
                "is-right",
                "is-small",
                IF!(aol_edit =>"has-text-info")
            ],
            i![C!["fas", if aol_edit { "fa-check" } else { "fa-edit" }]]
        ]
    ]
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
    thoughts: &[&Thought],
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
                    let wait = wait_for_deletion.as_ref().is_some_and(|id| id == &t.id);
                    li![thought(t, wait)]
                })]
        }
    ]
}

fn thought(t: &Thought, wait_for_deletion: bool) -> Node<Msg> {
    let sel_id = t.id;
    let del_id = t.id;
    div![
        C!["level"],
        ev(Ev::Click, move |_| Msg::SelectRequest(sel_id)),
        div![C!["level-left"], div![C!["level-item"], &t.title]],
        div![
            C!["level-right"],
            div![
                C!["level-item"],
                button![
                    ev(Ev::Click, move |_| Msg::DeleteRequest(del_id)),
                    C!["button", "is-small", "is-danger"],
                    if wait_for_deletion {
                        C!["is-loading"]
                    } else {
                        C![]
                    },
                    span![C!["icon"], i![C!["fa", "fa-trash-alt"],]],
                ]
            ]
        ]
    ]
}
