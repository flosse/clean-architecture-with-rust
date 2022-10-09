use crate::storage::data_storage;
use cawr_adapter::{api::Api, presenter::cli::Presenter};
use clap::{Parser, Subcommand};
use std::{collections::HashSet, path::PathBuf, sync::Arc};

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
    #[clap(help = "Directory to store data ", long)]
    data_dir: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    #[clap(about = "Create a new thought")]
    Create { title: String },
    #[clap(about = "Read an specific thought")]
    Read { id: String },
}

pub fn run() {
    let args = Args::parse();
    let db = Arc::new(data_storage(args.data_dir));
    let app_api = Api::new(db, Presenter::default());

    match args.command {
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
