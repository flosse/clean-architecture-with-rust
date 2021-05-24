use adapter::{
    controller::item::{
        create::Controller as CreateController, find_by_id::Controller as FindController,
    },
    id::item::ItemId,
    presenter::cli::Presenter,
};
use application::gateway::repository::item::ItemRepo;
use std::sync::Arc;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Cmd {
    #[structopt(about = "Create a new item")]
    Create { title: String },
    #[structopt(about = "Read an item")]
    Read { id: String },
}

pub fn run<R>(repo: R)
where
    R: ItemRepo<Id = ItemId> + 'static,
{
    let cmd = Cmd::from_args();
    let repo = Arc::new(repo);
    let presenter = Presenter::default();

    match cmd {
        Cmd::Create { title } => {
            let controller = CreateController::new(repo, presenter);
            match controller.create_item(title) {
                Ok(res) => {
                    println!("Created a new item (ID = {})", res);
                }
                Err(err) => {
                    println!("Undable to create a new item: {}", err);
                }
            }
        }
        Cmd::Read { id } => {
            let controller = FindController::new(repo, presenter);
            match controller.find_item(&id) {
                Ok(item) => {
                    println!("{}", item);
                }
                Err(err) => {
                    println!("Undable find item: {}", err);
                }
            }
        }
    }
}
