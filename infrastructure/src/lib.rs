use db::json_file::JsonFile;
use std::env;

pub mod cli;
pub mod web;

pub fn run() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    let db = JsonFile::try_new().expect("JSON file store");
    cli::run(db);
}
