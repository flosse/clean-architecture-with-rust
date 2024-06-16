use std::{collections::HashSet, sync::Arc};

use clap::Subcommand;

use cawr_adapter::{api::Api, db::Db, presenter::cli::Presenter};

#[derive(Subcommand)]
pub enum Command {
    #[clap(about = "Create a new thought")]
    Create { title: String },
    #[clap(about = "Read an specific thought")]
    Read { id: String },
}

pub fn run<D>(db: Arc<D>, cmd: Command)
where
    D: Db,
{
    let app_api = Api::new(db, Presenter::default());

    match cmd {
        Command::Create { title } => {
            let areas_of_life = HashSet::new(); // Areas of life needs to be added later
            let res = app_api.create_thought(title, &areas_of_life);
            println!("{}", res);
        }
        Command::Read { id } => {
            let res = app_api.find_thought(&id);
            println!("{}", res);
        }
    }
}
