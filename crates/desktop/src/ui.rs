use cawr_adapter::model::view::json::{area_of_life::AreaOfLife, thought::Thought};
use eframe::egui;

// ----- ------
//    Model
// ----- ------

#[derive(Default, Debug)]
pub struct Mdl {
    thoughts: Vec<Thought>,
    areas_of_life: Vec<AreaOfLife>,
}

// ----- ------
//   Messages
// ----- ------

#[derive(Debug)]
pub enum Msg {
    ThoughtsChanged(Vec<Thought>),
    AreasOfLifeChanged(Vec<AreaOfLife>),
}

// ----- ------
//   Commands
// ----- ------

#[derive(Debug)]
pub enum Cmd {
    ReadAllAreasOfLife,
    ReadAllThoughts,
}

// ----- ------
//    Update
// ----- ------

pub fn update(msg: Msg, mdl: &mut Mdl) {
    log::debug!("update model");
    match msg {
        Msg::ThoughtsChanged(data) => {
            mdl.thoughts = data;
        }
        Msg::AreasOfLifeChanged(data) => {
            mdl.areas_of_life = data;
        }
    }
}

// ----- ------
//    View
// ----- ------

pub fn view(mdl: &mut Mdl, ctx: &egui::Context) -> Vec<Cmd> {
    let cmds = vec![];
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        for aol in &mdl.areas_of_life {
            ui.label(&aol.name);
        }
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        for t in &mdl.thoughts {
            ui.label(&t.title);
        }
    });
    cmds
}
