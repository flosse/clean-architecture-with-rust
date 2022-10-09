use cawr_application::{gateway::repository as repo, identifier::NewId};

pub trait Db:
    repo::thought::Repo
    + NewId<cawr_domain::thought::Id>
    + repo::area_of_life::Repo
    + NewId<cawr_domain::area_of_life::Id>
    + 'static
{
}
