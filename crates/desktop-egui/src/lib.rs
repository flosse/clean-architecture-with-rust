use anyhow::{anyhow, Result};
use cawr_adapter::{api, db::Db, presenter::http_json_api::Presenter};
use eframe::egui;
use std::sync::{mpsc, Arc};
use tokio::runtime;

mod actions;
mod ui;

// ----- ------
//    Start
// ----- ------

const TITLE: &str = "Clean Architecture with Rust";

pub fn run<D>(db: Arc<D>) -> Result<()>
where
    D: Db,
{
    log::debug!("Start desktop application");
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let app_api = Api::new(db, Presenter);
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        TITLE,
        options,
        Box::new(|cc| {
            let ctx = cc.egui_ctx.clone();
            let mut app = App::new(app_api, rt, ctx);
            let init_cmds = vec![ui::Cmd::ReadAllAreasOfLife, ui::Cmd::ReadAllThoughts];
            handle_commands(init_cmds, &mut app);
            Box::new(app)
        }),
    )
    .map_err(|err| anyhow!("Unable to start dektop application: {err}"))
}

// ----- ------
//    Model
// ----- ------

type Api<D> = api::Api<D, Presenter>;

struct App<D> {
    ui: ui::Mdl,
    api: Api<D>,
    egui: egui::Context,
    rt: runtime::Runtime,
    msg_tx: mpsc::Sender<ui::Msg>,
    msg_rx: mpsc::Receiver<ui::Msg>,
}

impl<D> App<D>
where
    D: Db,
{
    fn new(api: Api<D>, rt: runtime::Runtime, egui_ctx: egui::Context) -> Self {
        let ui = ui::Mdl::default();
        let (msg_tx, msg_rx) = mpsc::channel();
        let egui = egui_ctx;
        Self {
            api,
            egui,
            ui,
            rt,
            msg_tx,
            msg_rx,
        }
    }
    fn spawn_action<F>(&self, f: F)
    where
        F: Fn(Api<D>) -> Option<ui::Msg> + Send + 'static,
    {
        let tx = self.msg_tx.clone();
        let api = self.api.clone();
        let egui = self.egui.clone();
        self.rt.spawn(async move {
            if let Some(msg) = f(api) {
                tx.send(msg).unwrap();
                egui.request_repaint();
            }
        });
    }
}

impl<D> eframe::App for App<D>
where
    D: Db,
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.msg_rx.try_recv() {
            Ok(msg) => {
                ui::update(msg, &mut self.ui);
            }
            Err(mpsc::TryRecvError::Empty) => { /* nothing to do */ }
            Err(mpsc::TryRecvError::Disconnected) => {
                log::error!("Unable to receive messages");
            }
        }
        let cmds = ui::view(&mut self.ui, ctx);
        handle_commands(cmds, self);
    }
}

// ----- ------
//  UI Commands
// ----- ------

fn handle_commands<D>(cmds: Vec<ui::Cmd>, app: &mut App<D>)
where
    D: Db,
{
    for cmd in cmds {
        log::debug!("Handle UI command {cmd:?}");
        match cmd {
            ui::Cmd::ReadAllAreasOfLife => {
                app.spawn_action(actions::read_all_areas_of_life);
            }
            ui::Cmd::ReadAllThoughts => {
                app.spawn_action(actions::read_all_thoughts);
            }
        }
    }
}
