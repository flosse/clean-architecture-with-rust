use crate::web::handler::{reply_error, Result};
use adapter::{
    controller::thought::Controller, db::Db, model::view::json::thought::create::Request,
    presenter::http_json_api::Presenter,
};
use std::sync::Arc;
use warp::{reply, Reply};

pub async fn handle<D>(
    req: Request,
    controller: Arc<Controller<D, Presenter>>,
) -> Result<impl Reply>
where
    D: Db,
{
    match controller.create_thought(req.title) {
        Ok(res) => Ok(reply::with_status(reply::json(&res.data), res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::{handle, Arc, Controller, Presenter, Request};
    use crate::web::tests::{blank_db, response_json_body};
    use adapter::model::view::json::{thought::create as uc, Error};
    use application::gateway::repository::thought::Repo;
    use serde_json::Value;
    use warp::{http::StatusCode, Reply};

    #[tokio::test]
    async fn create() {
        let db = blank_db();
        let controller = Arc::new(Controller::new(db.clone(), Presenter::default()));
        let req = Request {
            title: "test 1".to_string(),
        };
        let res = handle(req, controller).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::CREATED);

        let body: Value = response_json_body(res).await.unwrap();
        let id = body.as_str().unwrap().parse().unwrap();
        let record = db.as_ref().get(id).unwrap();

        assert_eq!(record.thought.title.as_ref(), "test 1");
    }

    #[tokio::test]
    async fn create_with_too_short_title() {
        let db = blank_db();
        let controller = Arc::new(Controller::new(db, Presenter::default()));
        let req = Request {
            title: "t".to_string(),
        };
        let res = handle(req, controller).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let err: Error<uc::Error> = response_json_body(res).await.unwrap();

        assert_eq!(
            err.msg.unwrap(),
            "The title must have at least 3 but has 1 chars"
        );
        assert_eq!(err.status, StatusCode::BAD_REQUEST);
        assert!(matches!(
            err.details.unwrap(),
            uc::Error::TitleMinLength { actual: 1, min: 3 }
        ));
    }

    #[tokio::test]
    async fn create_with_too_long_title() {
        let db = blank_db();
        let controller = Arc::new(Controller::new(db, Presenter::default()));
        let req = Request {
            title: ["t"; 100].join(""),
        };
        let res = handle(req, controller).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let err: Error<uc::Error> = response_json_body(res).await.unwrap();

        assert_eq!(
            err.msg.unwrap(),
            "The title must have at most 80 but has 100 chars"
        );
        assert_eq!(err.status, StatusCode::BAD_REQUEST);
        assert!(matches!(
            err.details.unwrap(),
            uc::Error::TitleMaxLength {
                actual: 100,
                max: 80
            }
        ));
    }
}
