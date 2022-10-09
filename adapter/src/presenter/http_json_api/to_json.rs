use crate::model::{app, view::json};

impl From<app::area_of_life::Id> for json::area_of_life::AreaOfLifeId {
    fn from(from: app::area_of_life::Id) -> Self {
        from.to_u64().into()
    }
}

impl From<app::thought::Id> for json::thought::ThoughtId {
    fn from(from: app::thought::Id) -> Self {
        from.to_u64().into()
    }
}

pub(crate) mod thought {
    pub mod create {
        use crate::model::{
            app::thought::create::Response, view::json::thought::create::Error,
            view::json::thought::ThoughtId,
        };
        use cawr_application::usecase::thought::validate::{self, ThoughtInvalidity};

        pub fn thought_id_from_response(res: Response) -> ThoughtId {
            res.id.to_u64().into()
        }

        pub fn from_thought_invalidity(from: ThoughtInvalidity) -> Error {
            let ThoughtInvalidity::Title(e) = from;
            use validate::TitleInvalidity as T;
            match e {
                T::MinLength { min, actual } => Error::TitleMinLength { min, actual },
                T::MaxLength { max, actual } => Error::TitleMaxLength { max, actual },
            }
        }
    }
    pub mod update {
        use crate::model::view::json::thought::update::Error;
        use cawr_application::usecase::thought::validate::{self, ThoughtInvalidity};

        pub fn from_thought_invalidity(from: ThoughtInvalidity) -> Error {
            let ThoughtInvalidity::Title(e) = from;
            use validate::TitleInvalidity as T;
            match e {
                T::MinLength { min, actual } => Error::TitleMinLength { min, actual },
                T::MaxLength { max, actual } => Error::TitleMaxLength { max, actual },
            }
        }
    }
    pub mod read_all {
        use crate::model::view::json::thought::Thought;
        use cawr_application::usecase::thought::read_all as uc;

        pub fn from_thought(from: uc::Thought) -> Thought {
            let uc::Thought {
                id,
                title,
                areas_of_life,
            } = from;
            let id = id.to_u64().into();
            let areas_of_life = areas_of_life
                .into_iter()
                .map(|id| id.to_u64().into())
                .collect();
            Thought {
                id,
                title,
                areas_of_life,
            }
        }
    }
    pub mod find_by_id {
        use crate::model::view::json::thought::Thought;
        use cawr_application::usecase::thought::find_by_id as uc;

        pub fn from_response(from: uc::Response) -> Thought {
            let uc::Response {
                id,
                title,
                areas_of_life,
            } = from;
            let id = id.to_u64().into();
            let areas_of_life = areas_of_life
                .into_iter()
                .map(|id| id.to_u64().into())
                .collect();
            Thought {
                id,
                title,
                areas_of_life,
            }
        }
    }
}

pub(crate) mod area_of_life {
    pub mod create {
        use crate::model::view::json::area_of_life::{create::Error, AreaOfLifeId};
        use cawr_application::usecase::area_of_life::{create as uc, validate};

        pub fn from_response(from: uc::Response) -> AreaOfLifeId {
            from.id.to_u64().into()
        }

        pub fn try_from_error(from: uc::Error) -> Result<Error, ()> {
            match from {
                uc::Error::Repo | uc::Error::NewId => Err(()),
                uc::Error::Invalidity(e) => {
                    let validate::AreaOfLifeInvalidity::Name(e) = e;
                    use validate::NameInvalidity as T;
                    Ok(match e {
                        T::MinLength { min, actual } => Error::NameMinLength { min, actual },
                        T::MaxLength { max, actual } => Error::NameMaxLength { max, actual },
                    })
                }
            }
        }
    }
    pub mod update {
        use crate::model::view::json::area_of_life::update::Error;
        use cawr_application::usecase::area_of_life::validate::{
            AreaOfLifeInvalidity, NameInvalidity,
        };

        pub fn from_area_of_life_invalidity(from: AreaOfLifeInvalidity) -> Error {
            let AreaOfLifeInvalidity::Name(e) = from;
            use NameInvalidity as T;
            match e {
                T::MinLength { min, actual } => Error::NameMinLength { min, actual },
                T::MaxLength { max, actual } => Error::NameMaxLength { max, actual },
            }
        }
    }
    pub mod read_all {
        use crate::model::view::json::area_of_life::AreaOfLife;
        use cawr_application::usecase::area_of_life::read_all as uc;

        pub fn from_area_of_life(from: uc::AreaOfLife) -> AreaOfLife {
            let uc::AreaOfLife { id, name } = from;
            let id = id.to_u64().into();
            AreaOfLife { id, name }
        }
    }
}
