use db::in_memory::InMemory;

pub mod cli;
pub mod db;

pub fn run() {
    let db = InMemory::default();
    cli::run(db);
}
