use adapter::{controller::item::ItemController, id::item::ItemId, presenter::item::CliPresenter};
use application::gateway::repository::item::ItemRepo;
use std::error::Error;
use structopt::StructOpt;

type RepoErr<D> = <D as ItemRepo>::Err;

#[derive(StructOpt)]
enum Cmd {
    #[structopt(about = "Create a new item")]
    Create { title: String },
}

pub fn run<D>(db: D)
where
    D: ItemRepo<Id = ItemId> + 'static,
    RepoErr<D>: Error + 'static,
{
    let cmd = Cmd::from_args();
    match cmd {
        Cmd::Create { title } => {
            let presenter = CliPresenter::default();
            let controller = ItemController::new(db, presenter);
            match controller.create_item(title) {
                Ok(res) => {
                    println!("Created a new item (ID = {})", res);
                }
                Err(err) => {
                    println!("Undable to create a new item: {}", err);
                }
            }
        }
    }
}
