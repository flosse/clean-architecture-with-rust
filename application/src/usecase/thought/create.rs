use crate::{
    gateway::repository::thought::{NewId, NewIdError, Repo, SaveError, ThoughtRecord},
    usecase::thought::validate::{validate_thought, ThoughtInvalidity},
};
use domain::thought::Thought;
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    /// The title of new thought.
    pub title: String,
}

#[derive(Debug)]
pub struct Response<Id> {
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

type Id<R> = <R as Repo>::Id;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("{}", NewIdError)]
    NewId,
    #[error(transparent)]
    Invalidity(#[from] ThoughtInvalidity),
}

impl From<SaveError> for Error {
    fn from(e: SaveError) -> Self {
        match e {
            SaveError::Connection => Self::Repo,
        }
    }
}

impl<'r, 'g, R, G> CreateThought<'r, 'g, R, G>
where
    R: Repo,
    G: NewId<Id<R>>,
    Id<R>: Clone + Copy,
{
    /// Create a new thought with the given title.
    pub fn exec(&self, req: Request) -> Result<Response<Id<R>>, Error> {
        log::debug!("Create new thought: {:?}", req);
        let thought = Thought::new(req.title);
        validate_thought(&thought)?;
        let id = self.id_gen.new_id().map_err(|err| {
            log::warn!("{}", err);
            Error::NewId
        })?;
        let record = ThoughtRecord { id, thought };
        self.repo.save(record)?;
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
        thought: RwLock<Option<ThoughtRecord<u32>>>,
    }

    impl Repo for MockRepo {
        type Id = u32;

        fn save(&self, record: ThoughtRecord<Self::Id>) -> Result<(), SaveError> {
            *self.thought.write().unwrap() = Some(record);
            Ok(())
        }
        fn get(&self, _: Self::Id) -> Result<ThoughtRecord<Self::Id>, GetError> {
            todo!()
        }
        fn get_all(&self) -> Result<Vec<ThoughtRecord<Self::Id>>, GetAllError> {
            todo!()
        }
        fn delete(&self, _: Self::Id) -> Result<(), DeleteError> {
            todo!()
        }
    }

    struct IdGen;

    impl NewId<u32> for IdGen {
        fn new_id(&self) -> Result<u32, NewIdError> {
            Ok(42)
        }
    }

    #[test]
    fn create_new_thought() {
        let repo = MockRepo::default();
        let gen = IdGen {};
        let usecase = CreateThought::new(&repo, &gen);
        let req = Request {
            title: "foo".into(),
        };
        let res = usecase.exec(req).unwrap();
        assert_eq!(
            repo.thought
                .read()
                .unwrap()
                .as_ref()
                .unwrap()
                .thought
                .title
                .as_ref(),
            "foo"
        );
        assert_eq!(res.id, 42);
    }

    #[test]
    fn create_with_empty_title() {
        let repo = MockRepo::default();
        let gen = IdGen {};
        let usecase = CreateThought::new(&repo, &gen);
        let req = Request { title: "".into() };
        let err = usecase.exec(req).err().unwrap();
        assert!(matches!(err, Error::Invalidity(_)));
    }
}
