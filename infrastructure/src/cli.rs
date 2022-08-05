use crate::web;
use adapter::{controller::thought::Controller as ThoughtController, presenter::cli::Presenter};
use clap::{Parser, Subcommand};
use db::json_file::JsonFile;
use directories::UserDirs;
use std::{
    collections::HashSet,
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::runtime::Runtime;

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
    #[clap(about = "Run web service")]
    Serve {
        #[clap(default_value = "127.0.0.1", help = "IP address", long)]
        bind: IpAddr,
        #[clap(default_value = "3030", help = "TCP port", long)]
        port: u16,
    },
}

pub fn run() {
    let args = Args::parse();
    let data_dir = data_storage_directory(args.data_dir);
    log::info!("Use data directory: {data_dir:?}");
    let db = JsonFile::try_new(data_dir).expect("JSON file store");
    let db = Arc::new(db);
    let presenter = Presenter::default();

    match args.command {
        Command::Create { title } => {
            let controller = ThoughtController::new(db, presenter);
            let areas_of_life = HashSet::new(); // Areas of life needs to be added later
            let res = controller.create_thought(title, &areas_of_life);
            println!("{}", res);
        }
        Command::Read { id } => {
            let controller = ThoughtController::new(db, presenter);
            let res = controller.find_thought(&id);
            println!("{}", res);
        }
        Command::Serve { bind, port } => {
            let rt = Runtime::new().expect("tokio runtime");
            let addr = SocketAddr::from((bind, port));
            rt.block_on(web::run(db, addr));
        }
    }
}

const DEFAULT_STORAGE_DIR_NAME: &str = "clean-architecture-with-rust-data";

// Get storage directory with the following priority:
// 1. Custom (passed by the CLI)
// 2. HOME/DOCUMENTS/clean-architecture-with-rust-data
// 3. HOME/clean-architecture-with-rust-data
// 4. Relative to the executable: ./clean-architecture-with-rust-data
pub fn data_storage_directory(data_dir: Option<PathBuf>) -> PathBuf {
    if let Some(data_dir) = data_dir {
        data_dir
    } else {
        let base_path = if let Some(users_dir) = UserDirs::new() {
            users_dir
                .document_dir()
                .unwrap_or_else(|| users_dir.home_dir())
                .to_path_buf()
        } else {
            Path::new(".").to_path_buf()
        };
        base_path.join(DEFAULT_STORAGE_DIR_NAME)
    }
}
