use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};

use clap::Parser;
use tokio::runtime::Runtime;

use crate::storage::data_storage;

#[derive(Parser)]
struct Args {
    #[clap(default_value = "127.0.0.1", help = "IP address", long)]
    bind: IpAddr,
    #[clap(default_value = "3030", help = "TCP port", long)]
    port: u16,
    #[clap(help = "Directory to store data ", long)]
    data_dir: Option<PathBuf>,
}

pub fn run() {
    let args = Args::parse();
    let db = Arc::new(data_storage(args.data_dir));
    let rt = Runtime::new().expect("tokio runtime");
    let addr = SocketAddr::from((args.bind, args.port));
    rt.block_on(cawr_web_server_warp::run(db, addr));
}
