use crate::web::handler::{reply_error, Result};
use adapter::{
    controller::thought::create::Controller,
    model::{app::thought::Id, view::json::thought::create as view},
    presenter::http_json_api::Presenter,
};
use application::gateway::repository::thought::{NewId, Repo};
use std::sync::Arc;
use warp::{reply, Reply};

pub async fn handle<R>(req: view::Request, repo: Arc<R>) -> Result<impl Reply>
where
    R: Repo<Id = Id> + 'static + NewId<Id>,
{
    let presenter = Presenter::default();
    let controller = Controller::new(Arc::clone(&repo), repo, presenter);
    match controller.create_thought(req.title) {
        Ok(res) => Ok(reply::with_status(reply::json(&res.data), res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::{handle, view::Request};
    use crate::web::tests::{blank_db, response_json_body};
    use adapter::model::view::json::{thought::create as uc, Error};
    use application::gateway::repository::thought::Repo;
    use serde_json::Value;
    use warp::{http::StatusCode, Reply};

    #[tokio::test]
    async fn create() {
        let db = blank_db();
        let req = Request {
            title: "test 1".to_string(),
        };
        let res = handle(req, db.clone()).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::CREATED);

        let body: Value = response_json_body(res).await.unwrap();
        let id = body.as_str().unwrap().parse().unwrap();
        let record = db.as_ref().get(id).unwrap();

        assert_eq!(record.thought.title.as_ref(), "test 1");
    }

    #[tokio::test]
    async fn create_with_too_short_title() {
        let db = blank_db();
        let req = Request {
            title: "t".to_string(),
        };
        let res = handle(req, db.clone()).await.unwrap().into_response();

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
        let req = Request {
            title: ["t"; 100].join(""),
        };
        let res = handle(req, db.clone()).await.unwrap().into_response();

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
