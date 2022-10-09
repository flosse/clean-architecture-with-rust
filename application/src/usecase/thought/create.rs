use crate::{
    gateway::repository::{
        area_of_life,
        thought::{self, Record, SaveError},
    },
    identifier::{NewId, NewIdError},
    usecase::{
        area_of_life::check_existence::{self as check_aol, CheckAreasOfLifeExistence},
        thought::validate::{self, validate_thought_properties, ThoughtInvalidity},
    },
};
use cawr_domain::{
    area_of_life as aol,
    thought::{Id, Thought, Title},
};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    /// The title of new thought.
    pub title: String,
    /// Associated [aol::AreaOfLife]s.
    pub areas_of_life: HashSet<aol::Id>,
}

#[derive(Debug)]
pub struct Response {
    /// The ID of the newly created thought.
    pub id: Id,
}

/// Create thought usecase interactor
pub struct CreateThought<'r, 'g, R, G> {
    repo: &'r R,
    id_gen: &'g G,
}

impl<'r, 'g, R, G> CreateThought<'r, 'g, R, G> {
    pub fn new(repo: &'r R, id_gen: &'g G) -> Self {
        Self { repo, id_gen }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("{}", NewIdError)]
    NewId,
    #[error(transparent)]
    Invalidity(#[from] ThoughtInvalidity),
    #[error("Areas of life {0:?} not found")]
    AreasOfLifeNotFound(HashSet<aol::Id>),
}

impl From<SaveError> for Error {
    fn from(e: SaveError) -> Self {
        match e {
            SaveError::Connection => Self::Repo,
        }
    }
}

impl From<check_aol::Error> for Error {
    fn from(e: check_aol::Error) -> Self {
        use check_aol::Error as E;
        match e {
            E::Repo => Error::Repo,
            E::NotFound(aol_ids) => Error::AreasOfLifeNotFound(aol_ids),
        }
    }
}

impl<'r, 'g, R, G> CreateThought<'r, 'g, R, G>
where
    R: thought::Repo + area_of_life::Repo,
    G: NewId<Id>,
{
    /// Create a new thought with the given title.
    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("Create new thought: {:?}", req);
        validate_thought_properties(&validate::Request { title: &req.title })?;
        CheckAreasOfLifeExistence::new(self.repo).exec(&req.areas_of_life)?;
        let title = Title::new(req.title);
        let id = self.id_gen.new_id().map_err(|err| {
            log::warn!("{}", err);
            Error::NewId
        })?;
        let thought = Thought::new(id, title, req.areas_of_life);
        let record = Record { thought };
        thought::Repo::save(self.repo, record)?;
        Ok(Response { id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gateway::repository::thought::{DeleteError, GetAllError, GetError};
    use std::sync::RwLock;

    #[derive(Default)]
    struct MockRepo {
        thought: RwLock<Option<Record>>,
    }

    impl thought::Repo for MockRepo {
        fn save(&self, record: Record) -> Result<(), SaveError> {
            *self.thought.write().unwrap() = Some(record);
            Ok(())
        }
        fn get(&self, _: Id) -> Result<Record, GetError> {
            todo!()
        }
        fn get_all(&self) -> Result<Vec<Record>, GetAllError> {
            todo!()
        }
        fn delete(&self, _: Id) -> Result<(), DeleteError> {
            todo!()
        }
    }

    impl area_of_life::Repo for MockRepo {
        fn save(&self, _: area_of_life::Record) -> Result<(), area_of_life::SaveError> {
            todo!()
        }
        fn get(&self, _: aol::Id) -> Result<area_of_life::Record, area_of_life::GetError> {
            todo!()
        }
        fn get_all(&self) -> Result<Vec<area_of_life::Record>, area_of_life::GetAllError> {
            todo!()
        }
        fn delete(&self, _: aol::Id) -> Result<(), area_of_life::DeleteError> {
            todo!()
        }
    }

    struct IdGen;

    impl NewId<Id> for IdGen {
        fn new_id(&self) -> Result<Id, NewIdError> {
            Ok(Id::new(42))
        }
    }

    #[test]
    fn create_new_thought() {
        let repo = MockRepo::default();
        let gen = IdGen {};
        let usecase = CreateThought::new(&repo, &gen);
        let req = Request {
            title: "foo".into(),
            areas_of_life: HashSet::new(),
        };
        let res = usecase.exec(req).unwrap();
        assert_eq!(
            repo.thought
                .read()
                .unwrap()
                .as_ref()
                .unwrap()
                .thought
                .title()
                .as_ref(),
            "foo"
        );
        assert_eq!(res.id, Id::new(42));
    }

    #[test]
    fn create_with_empty_title() {
        let repo = MockRepo::default();
        let gen = IdGen {};
        let usecase = CreateThought::new(&repo, &gen);
        let req = Request {
            title: "".into(),
            areas_of_life: HashSet::new(),
        };
        let err = usecase.exec(req).err().unwrap();
        assert!(matches!(err, Error::Invalidity(_)));
    }
}
