use crate::web;
use adapter::{
    controller::thought::Controller as ThoughtController, db::Db, presenter::cli::Presenter,
};
use clap::Parser;
use std::{
    collections::HashSet,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use tokio::runtime::Runtime;

#[derive(Parser)]
enum Cmd {
    #[clap(about = "Create a new thought")]
    Create { title: String },
    #[clap(about = "Read an specific thought")]
    Read { id: String },
    #[clap(about = "Run web service")]
    Serve {
        #[clap(default_value = "127.0.0.1", help = "IP address", long)]
        bind: IpAddr,
        #[clap(default_value = "3030", help = "TCP port", long)]
        port: u16,
    },
}

pub fn run<D>(db: D)
where
    D: Db,
{
    let cmd = Cmd::parse();
    let db = Arc::new(db);
    let presenter = Presenter::default();

    match cmd {
        Cmd::Create { title } => {
            let controller = ThoughtController::new(db, presenter);
            let areas_of_life = HashSet::new(); // Areas of life needs to be added later
            let res = controller.create_thought(title, &areas_of_life);
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
