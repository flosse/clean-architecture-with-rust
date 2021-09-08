use crate::model::app;
use application::{gateway::repository as repo, identifier::NewId};

pub trait Db:
    repo::thought::Repo<Id = app::thought::Id>
    + NewId<app::thought::Id>
    + repo::area_of_life::Repo<Id = app::area_of_life::Id>
    + NewId<app::area_of_life::Id>
    + 'static
{
}
