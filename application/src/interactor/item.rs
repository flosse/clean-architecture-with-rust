use crate::gateway::repository::item::ItemRepo;
use domain::usecase::item::create::{CreateItem, Request, Response};
use entity::item::Item;

pub struct Interactor<'r, R> {
    repo: &'r R,
}

impl<'r, R> Interactor<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

type Id<R> = <R as ItemRepo>::Id;
type Error<R> = <R as ItemRepo>::Err;

impl<R> CreateItem for Interactor<'_, R>
where
    R: ItemRepo,
{
    type Err = Error<R>;
    type Id = Id<R>;
    fn exec(&self, req: Request) -> Result<Response<Self::Id>, Self::Err> {
        let item = Item { title: req.title };
        let id = self.repo.save(item)?;
        Ok(Response { id })
    }
}
