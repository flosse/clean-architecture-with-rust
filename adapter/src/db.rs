use application::{gateway::repository as repo, identifier::NewId};

pub trait Db:
    repo::thought::Repo
    + NewId<domain::thought::Id>
    + repo::area_of_life::Repo
    + NewId<domain::area_of_life::Id>
    + 'static
{
}
