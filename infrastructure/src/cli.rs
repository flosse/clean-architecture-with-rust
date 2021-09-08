use crate::web;
use adapter::{
    controller::thought::Controller as ThoughtController, db::Db, presenter::cli::Presenter,
};
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use structopt::StructOpt;
use tokio::runtime::Runtime;

#[derive(StructOpt)]
enum Cmd {
    #[structopt(about = "Create a new thought")]
    Create { title: String },
    #[structopt(about = "Read an specific thought")]
    Read { id: String },
    #[structopt(about = "Run web service")]
    Serve {
        #[structopt(default_value = "127.0.0.1", help = "IP address", long)]
        bind: IpAddr,
        #[structopt(default_value = "3030", help = "TCP port", long)]
        port: u16,
    },
}

pub fn run<D>(db: D)
where
    D: Db,
{
    let cmd = Cmd::from_args();
    let db = Arc::new(db);
    let presenter = Presenter::default();

    match cmd {
        Cmd::Create { title } => {
            let controller = ThoughtController::new(db, presenter);
            let res = controller.create_thought(title);
            println!("{}", res);
        }
        Cmd::Read { id } => {
            let controller = ThoughtController::new(db, presenter);
            let res = controller.find_thought(&id);
            println!("{}", res);
        }
        Cmd::Serve { bind, port } => {
            let rt = Runtime::new().expect("tokio runtime");
            let addr = SocketAddr::from((bind, port));
            rt.block_on(web::run(db, addr));
        }
    }
}
