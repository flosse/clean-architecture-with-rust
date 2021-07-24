use crate::web::handler::{reply_error, Result};
use adapter::{
    controller::thought::find_by_id::Controller, id::thought::Id, presenter::json::Presenter,
};
use application::gateway::repository::thought::Repo;
use std::sync::Arc;
use warp::{http::StatusCode, reply, Reply};

pub type Request = String;

pub async fn handle<R>(req: Request, repo: Arc<R>) -> Result<impl Reply>
where
    R: Repo<Id = Id> + 'static,
{
    let presenter = Presenter::default();
    let controller = Controller::new(repo, presenter);
    match controller.find_thought(&req) {
        Ok(res) => Ok(reply::with_status(reply::json(&res), StatusCode::OK)),
        Err(err) => Ok(reply_error(err.into())),
    }
}

#[cfg(test)]
mod tests {
    use super::handle;
    use crate::web::{
        handler::JsonError,
        tests::{add_thought_to_db, blank_db, corrupt_db, response_json_body},
    };
    use serde_json::Value;
    use warp::{http::StatusCode, Reply};

    #[tokio::test]
    async fn read() {
        let db = blank_db();
        add_thought_to_db(&db, "foo");
        add_thought_to_db(&db, "bar");

        let req = "2".to_string();
        let res = handle(req, db.clone()).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::OK);

        let body: Value = response_json_body(res).await.unwrap();
        let thought = body.as_object().unwrap();
        let title = thought.get("title").unwrap().as_str().unwrap();

        assert_eq!(title, "bar");
    }

    #[tokio::test]
    async fn read_non_existent() {
        let db = blank_db();

        let req = "5".to_string();
        let res = handle(req, db.clone()).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        let err: JsonError = response_json_body(res).await.unwrap();

        assert_eq!(err.msg, "Could not find thought");
        assert_eq!(err.status_code, 404);
    }

    #[tokio::test]
    async fn read_invalid_id() {
        let db = blank_db();

        let req = "invalid-id".to_string();
        let res = handle(req, db.clone()).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let err: JsonError = response_json_body(res).await.unwrap();
        assert_eq!(err.msg, "Unable to parse thought ID");
        assert_eq!(err.status_code, 400);
    }

    #[tokio::test]
    async fn read_with_corrupt_db() {
        let db = corrupt_db();

        let req = "1".to_string();
        let res = handle(req, db.clone()).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let err: JsonError = response_json_body(res).await.unwrap();

        assert_eq!(err.msg, "A database error occured");
        assert_eq!(err.status_code, 500);
    }
}
