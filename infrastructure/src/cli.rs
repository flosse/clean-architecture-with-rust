use crate::web;
use adapter::{
    controller::thought::{
        create::Controller as CreateController, find_by_id::Controller as FindController,
    },
    model::app::thought::Id,
    presenter::cli::Presenter,
};
use application::gateway::repository::thought::Repo;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::runtime::Runtime;

#[derive(StructOpt)]
enum Cmd {
    #[structopt(about = "Create a new thought")]
    Create { title: String },
    #[structopt(about = "Read an specific thought")]
    Read { id: String },
    #[structopt(about = "Run web service")]
    Serve {},
}

pub fn run<R>(repo: R)
where
    R: Repo<Id = Id> + 'static,
{
    let cmd = Cmd::from_args();
    let repo = Arc::new(repo);
    let presenter = Presenter::default();

    match cmd {
        Cmd::Create { title } => {
            let controller = CreateController::new(repo, presenter);
            let res = controller.create_thought(title);
            println!("{}", res);
        }
        Cmd::Read { id } => {
            let controller = FindController::new(repo, presenter);
            let res = controller.find_thought(&id);
            println!("{}", res);
        }
        Cmd::Serve {} => {
            let rt = Runtime::new().expect("tokio runtime");
            rt.block_on(web::run(repo));
        }
    }
}
