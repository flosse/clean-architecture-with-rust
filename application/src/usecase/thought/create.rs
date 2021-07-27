use crate::{
    gateway::repository::thought::{Error as RepoError, Repo},
    usecase::thought::validate::{validate_thought, ThoughtInvalidity},
};
use domain::thought::Thought;
use thiserror::Error;

pub struct Request {
    /// The title of new thought.
    pub title: String,
}

pub struct Response<Id> {
    /// The ID of the newly created thought.
    pub id: Id,
}

/// Create thought usecase interactor
pub struct CreateThought<'r, R> {
    repo: &'r R,
}

impl<'r, R> CreateThought<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

type Id<R> = <R as Repo>::Id;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Repo(#[from] RepoError),
    #[error(transparent)]
    Invalidity(#[from] ThoughtInvalidity),
}

impl<'r, R> CreateThought<'r, R>
where
    R: Repo,
{
    /// Create a new thought with the given title.
    pub fn exec(&self, req: Request) -> Result<Response<Id<R>>, Error> {
        let thought = Thought::new(req.title);
        validate_thought(&thought)?;
        let id = self.repo.save(thought)?;
        Ok(Response { id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::RwLock;

    #[derive(Default)]
    struct MockRepo {
        thought: RwLock<Option<Thought>>,
    }

    impl Repo for MockRepo {
        type Id = u32;

        fn save(&self, thought: Thought) -> Result<Self::Id, RepoError> {
            *self.thought.write().unwrap() = Some(thought);
            Ok(42)
        }
        fn get(&self, _: Self::Id) -> Result<Thought, RepoError> {
            todo!()
        }
    }

    #[test]
    fn create_new_thought() {
        let repo = MockRepo::default();
        let usecase = CreateThought::new(&repo);
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
                .title
                .as_ref(),
            "foo"
        );
        assert_eq!(res.id, 42);
    }

    #[test]
    fn create_with_empty_title() {
        let repo = MockRepo::default();
        let usecase = CreateThought::new(&repo);
        let req = Request { title: "".into() };
        let err = usecase.exec(req).err().unwrap();
        assert!(matches!(err, Error::Invalidity(_)));
    }
}
