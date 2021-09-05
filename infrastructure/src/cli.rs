use crate::web;
use adapter::{
    controller::thought::{
        create::Controller as CreateController, find_by_id::Controller as FindController,
    },
    model::app::thought::Id,
    presenter::cli::Presenter,
};
use application::{gateway::repository::thought::Repo, identifier::NewId};
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

pub fn run<R>(repo: R)
where
    R: Repo<Id = Id> + 'static + NewId<Id>,
{
    let cmd = Cmd::from_args();
    let repo = Arc::new(repo);
    let presenter = Presenter::default();

    match cmd {
        Cmd::Create { title } => {
            let controller = CreateController::new(Arc::clone(&repo), repo, presenter);
            let res = controller.create_thought(title);
            println!("{}", res);
        }
        Cmd::Read { id } => {
            let controller = FindController::new(repo, presenter);
            let res = controller.find_thought(&id);
            println!("{}", res);
        }
        Cmd::Serve { bind, port } => {
            let rt = Runtime::new().expect("tokio runtime");
            let addr = SocketAddr::from((bind, port));
            rt.block_on(web::run(repo, addr));
        }
    }
}
