use crate::{
    model::view::json::{Error, Response, Result, StatusCode},
    presenter::Present,
};

mod to_json;

#[derive(Default, Clone)]
pub struct Presenter;

mod thought {
    use super::{to_json, Error, Present, Presenter, Response, Result, StatusCode};
    use crate::model::{
        app::thought as app,
        view::json::{area_of_life::AreaOfLifeId, thought as view},
    };

    // -- Create -- //

    impl Present<app::create::Result> for Presenter {
        type ViewModel = Result<view::ThoughtId, view::create::Error>;
        fn present(&self, res: app::create::Result) -> Self::ViewModel {
            res.map(to_json::thought::create::thought_id_from_response)
                .map(|id| Response {
                    data: Some(id),
                    status: StatusCode::CREATED,
                })
                .map_err(|err| {
                    use app::create::Error as E;
                    match err {
                        E::AreaOfLifeId => Error {
                            msg: Some(err.to_string()),
                            status: StatusCode::BAD_REQUEST,
                            details: Some(view::create::Error::AreaOfLifeId),
                        },
                        E::Invalidity(invalidity) => Error {
                            msg: Some(invalidity.to_string()),
                            status: StatusCode::BAD_REQUEST,
                            details: Some(to_json::thought::create::from_thought_invalidity(
                                invalidity,
                            )),
                        },
                        E::AreasOfLifeNotFound(ref ids) => Error {
                            msg: Some(err.to_string()),
                            status: StatusCode::BAD_REQUEST,
                            details: Some(view::create::Error::AreasOfLifeNotFound(
                                ids.clone().into_iter().map(AreaOfLifeId::from).collect(),
                            )),
                        },
                        E::Repo | E::NewId => Error::internal(),
                    }
                })
        }
    }

    // -- Update -- //

    impl Present<app::update::Result> for Presenter {
        type ViewModel = Result<(), view::update::Error>;
        fn present(&self, res: app::update::Result) -> Self::ViewModel {
            res.map(|()| Response {
                data: None,
                status: StatusCode::OK,
            })
            .map_err(|err| {
                use app::update::Error as E;
                match err {
                    E::Id => Error {
                        msg: Some(err.to_string()),
                        status: StatusCode::BAD_REQUEST,
                        details: Some(view::update::Error::Id),
                    },
                    E::NotFound(id) => Error {
                        msg: Some(err.to_string()),
                        status: StatusCode::NOT_FOUND,
                        details: Some(view::update::Error::NotFound(id.into())),
                    },
                    E::AreaOfLifeId => Error {
                        msg: Some(err.to_string()),
                        status: StatusCode::BAD_REQUEST,
                        details: Some(view::update::Error::AreaOfLifeId),
                    },
                    E::Invalidity(invalidity) => Error {
                        msg: Some(invalidity.to_string()),
                        status: StatusCode::BAD_REQUEST,
                        details: Some(to_json::thought::update::from_thought_invalidity(
                            invalidity,
                        )),
                    },
                    E::AreasOfLifeNotFound(ref ids) => Error {
                        msg: Some(err.to_string()),
                        status: StatusCode::BAD_REQUEST,
                        details: Some(view::update::Error::AreasOfLifeNotFound(
                            ids.clone().into_iter().map(AreaOfLifeId::from).collect(),
                        )),
                    },
                    E::Repo => Error::internal(),
                }
            })
        }
    }

    // -- Find by ID -- //

    impl Present<app::find_by_id::Result> for Presenter {
        type ViewModel = Result<view::Thought, view::find_by_id::Error>;
        fn present(&self, res: app::find_by_id::Result) -> Self::ViewModel {
            res.map(to_json::thought::find_by_id::from_response)
                .map(|data| Response {
                    data: Some(data),
                    status: StatusCode::OK,
                })
                .map_err(|err| match err {
                    app::find_by_id::Error::Id => Error {
                        msg: Some(err.to_string()),
                        status: StatusCode::BAD_REQUEST,
                        details: Some(view::find_by_id::Error::Id),
                    },
                    app::find_by_id::Error::NotFound => Error {
                        msg: Some("Could not find thought".to_string()),
                        status: StatusCode::NOT_FOUND,
                        details: Some(view::find_by_id::Error::NotFound),
                    },
                    app::find_by_id::Error::Repo => Error::internal(),
                })
        }
    }

    // -- Read all -- //

    impl Present<app::read_all::Result> for Presenter {
        type ViewModel = Result<Vec<view::Thought>, view::read_all::Error>;
        fn present(&self, res: app::read_all::Result) -> Self::ViewModel {
            res.map(|resp| {
                resp.thoughts
                    .into_iter()
                    .map(to_json::thought::read_all::from_thought)
                    .collect()
            })
            .map(|data| Response {
                data: Some(data),
                status: StatusCode::OK,
            })
            .map_err(|err| match err {
                app::read_all::Error::Repo => Error::internal(),
            })
        }
    }

    // -- Delete by ID -- //

    impl Present<app::delete::Result> for Presenter {
        type ViewModel = Result<(), view::delete::Error>;
        fn present(&self, res: app::delete::Result) -> Self::ViewModel {
            res.map(|_| Response {
                data: None,
                status: StatusCode::OK,
            })
            .map_err(|err| match err {
                app::delete::Error::Id => Error {
                    msg: Some(err.to_string()),
                    status: StatusCode::BAD_REQUEST,
                    details: Some(view::delete::Error::Id),
                },
                app::delete::Error::NotFound => Error {
                    msg: Some("Could not find thought".to_string()),
                    status: StatusCode::NOT_FOUND,
                    details: Some(view::delete::Error::NotFound),
                },
                app::delete::Error::Repo => Error::internal(),
            })
        }
    }
}

mod area_of_life {
    use super::{to_json, Error, Present, Presenter, Response, Result, StatusCode};
    use crate::model::{app::area_of_life as app, view::json::area_of_life as view};

    // -- Create -- //

    impl Present<app::create::Result> for Presenter {
        type ViewModel = Result<view::AreaOfLifeId, view::create::Error>;
        fn present(&self, res: app::create::Result) -> Self::ViewModel {
            res.map(to_json::area_of_life::create::from_response)
                .map(|id| Response {
                    data: Some(id),
                    status: StatusCode::CREATED,
                })
                .map_err(|err| {
                    use app::create::Error as E;
                    match &err {
                        E::Invalidity(invalidity) => Error {
                            msg: Some(invalidity.to_string()),
                            status: StatusCode::BAD_REQUEST,
                            details: to_json::area_of_life::create::try_from_error(err).ok(),
                        },
                        E::Repo | E::NewId => Error::internal(),
                    }
                })
        }
    }

    // -- Update -- //

    impl Present<app::update::Result> for Presenter {
        type ViewModel = Result<(), view::update::Error>;
        fn present(&self, res: app::update::Result) -> Self::ViewModel {
            res.map(|()| Response {
                data: None,
                status: StatusCode::OK,
            })
            .map_err(|err| {
                use app::update::Error as E;
                match err {
                    E::Id => Error {
                        msg: Some(err.to_string()),
                        status: StatusCode::BAD_REQUEST,
                        details: Some(view::update::Error::Id),
                    },
                    E::NotFound(_) => Error {
                        msg: Some(err.to_string()),
                        status: StatusCode::NOT_FOUND,
                        details: Some(view::update::Error::NotFound),
                    },
                    E::Invalidity(invalidity) => Error {
                        msg: Some(invalidity.to_string()),
                        status: StatusCode::BAD_REQUEST,
                        details: Some(to_json::area_of_life::update::from_area_of_life_invalidity(
                            invalidity,
                        )),
                    },
                    E::Repo => Error::internal(),
                }
            })
        }
    }

    // -- Read all -- //

    impl Present<app::read_all::Result> for Presenter {
        type ViewModel = Result<Vec<view::AreaOfLife>, view::read_all::Error>;
        fn present(&self, res: app::read_all::Result) -> Self::ViewModel {
            res.map(|resp| {
                resp.areas_of_life
                    .into_iter()
                    .map(to_json::area_of_life::read_all::from_area_of_life)
                    .collect()
            })
            .map(|data| Response {
                data: Some(data),
                status: StatusCode::OK,
            })
            .map_err(|err| match err {
                app::read_all::Error::Repo => Error::internal(),
            })
        }
    }

    // -- Delete by ID -- //

    impl Present<app::delete::Result> for Presenter {
        type ViewModel = Result<(), view::delete::Error>;
        fn present(&self, res: app::delete::Result) -> Self::ViewModel {
            res.map(|_| Response {
                data: None,
                status: StatusCode::OK,
            })
            .map_err(|err| match err {
                app::delete::Error::Id => Error {
                    msg: Some(err.to_string()),
                    status: StatusCode::BAD_REQUEST,
                    details: Some(view::delete::Error::Id),
                },
                app::delete::Error::NotFound => Error {
                    msg: Some("Could not find area of life".to_string()),
                    status: StatusCode::NOT_FOUND,
                    details: Some(view::delete::Error::NotFound),
                },
                app::delete::Error::Repo => Error::internal(),
            })
        }
    }
}
