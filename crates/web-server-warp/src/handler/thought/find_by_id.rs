use crate::{
    handler::{reply_error, reply_json, Result},
    AppApi,
};
use cawr_adapter::db::Db;
use warp::Reply;

pub type Request = String;

pub async fn handle<D>(req: Request, api: AppApi<D>) -> Result<impl Reply>
where
    D: Db,
{
    match api.find_thought(&req) {
        Ok(res) => Ok(reply_json(&res.data, res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::handle;
    use crate::tests::{add_thought_to_db, app_api, blank_db, corrupt_db, response_json_body};
    use cawr_adapter::model::view::json::{self as json, thought::find_by_id as uc, Error};
    use serde_json::Value;
    use warp::{http::StatusCode, Reply};

    #[tokio::test]
    async fn read() {
        let db = blank_db();
        add_thought_to_db(&db, "foo");
        add_thought_to_db(&db, "bar");

        let app_api = app_api(db.clone());
        let req = "2".to_string();
        let res = handle(req, app_api).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::OK);

        let body: Value = response_json_body(res).await.unwrap();
        let thought = body.as_object().unwrap();
        let title = thought.get("title").unwrap().as_str().unwrap();

        assert_eq!(title, "bar");
    }

    #[tokio::test]
    async fn read_non_existent() {
        let db = blank_db();

        let app_api = app_api(db.clone());
        let req = "5".to_string();
        let res = handle(req, app_api).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        let err: Error<uc::Error> = response_json_body(res).await.unwrap();

        assert_eq!(err.msg.unwrap(), "Could not find thought");
        assert_eq!(err.status, json::StatusCode::NOT_FOUND);
        assert!(matches!(err.details.unwrap(), uc::Error::NotFound));
    }

    #[tokio::test]
    async fn read_invalid_id() {
        let db = blank_db();

        let app_api = app_api(db.clone());
        let req = "invalid-id".to_string();
        let res = handle(req, app_api).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let err: Error<uc::Error> = response_json_body(res).await.unwrap();
        assert_eq!(err.msg.unwrap(), "Unable to parse thought ID");
        assert_eq!(err.status, json::StatusCode::BAD_REQUEST);
        assert!(matches!(err.details.unwrap(), uc::Error::Id));
    }

    #[tokio::test]
    async fn read_with_corrupt_db() {
        let db = corrupt_db();

        let app_api = app_api(db.clone());
        let req = "1".to_string();
        let res = handle(req, app_api).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let err: Error<uc::Error> = response_json_body(res).await.unwrap();

        assert_eq!(err.msg, None);
        assert_eq!(err.status, json::StatusCode::INTERNAL_SERVER_ERROR);
        assert!(err.details.is_none());
    }
}
