use adapter::controller::item::ItemController;
use adapter::presenter::item::JsonPresenter;
use application::gateway::repository::item::ItemRepo;
use std::{error::Error, fmt::Display};

type RepoErr<D> = <D as ItemRepo>::Err;
type Id<D> = <D as ItemRepo>::Id;

pub fn run<D>(db: D)
where
    D: ItemRepo,
    Id<D>: Display,
    RepoErr<D>: Error + 'static,
{
    let presenter = JsonPresenter::default();
    let controller = ItemController::new(db, presenter);
    match controller.create_item("test") {
        Ok(res) => {
            println!("Created a new item ({})", res);
        }
        Err(err) => {
            println!("Undable to create a new item: {})", err);
        }
    }
}
