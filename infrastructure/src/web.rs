use crate::storage::data_storage;
use clap::Parser;
use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};
use tokio::runtime::Runtime;

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
    rt.block_on(web_server::run(db, addr));
}
