use db::json_file::JsonFile;

pub mod cli;
pub mod db;
pub mod web;

pub fn run() {
    pretty_env_logger::init();
    let db = JsonFile::try_new().expect("JSON file store");
    cli::run(db);
}
