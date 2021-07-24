use crate::web::handler::{reply_error, Result};
use adapter::controller::thought::create::Controller;
use adapter::{id::thought::Id, presenter::json::Presenter};
use application::gateway::repository::thought::Repo;
use serde::Deserialize;
use std::sync::Arc;
use warp::{http::StatusCode, reply, Reply};

#[derive(Deserialize)]
pub struct Request {
    title: String,
}

pub async fn handle<R>(req: Request, repo: Arc<R>) -> Result<impl Reply>
where
    R: Repo<Id = Id> + 'static,
{
    let presenter = Presenter::default();
    let controller = Controller::new(repo, presenter);
    match controller.create_thought(req.title) {
        Ok(res) => Ok(reply::with_status(reply::json(&res), StatusCode::CREATED)),
        Err(err) => Ok(reply_error(err.into())),
    }
}

#[cfg(test)]
mod tests {
    use super::{handle, Request};
    use crate::web::{
        handler::JsonError,
        tests::{blank_db, response_json_body},
    };
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
        let thought = db.as_ref().get(id).unwrap();

        assert_eq!(thought.title.as_ref(), "test 1");
    }

    #[tokio::test]
    async fn create_with_too_short_title() {
        let db = blank_db();
        let req = Request {
            title: "t".to_string(),
        };
        let res = handle(req, db.clone()).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let err: JsonError = response_json_body(res).await.unwrap();

        assert_eq!(err.msg, "The title must have at least 3 but has 1 chars");
        assert_eq!(err.status_code, 400);
    }

    #[tokio::test]
    async fn create_with_too_long_title() {
        let db = blank_db();
        let req = Request {
            title: ["t"; 100].join(""),
        };
        let res = handle(req, db.clone()).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let err: JsonError = response_json_body(res).await.unwrap();

        assert_eq!(err.msg, "The title must have at most 80 but has 100 chars");
        assert_eq!(err.status_code, 400);
    }
}
