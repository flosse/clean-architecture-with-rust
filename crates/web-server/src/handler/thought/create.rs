use crate::{
    handler::{reply_error, Result},
    AppApi,
};
use cawr_adapter::{db::Db, model::view::json::thought::create::Request};
use warp::{reply, Reply};

pub async fn handle<D>(req: Request, api: AppApi<D>) -> Result<impl Reply>
where
    D: Db,
{
    let areas_of_life = req
        .areas_of_life
        .into_iter()
        .map(|id| id.0.to_string())
        .collect();
    match api.create_thought(req.title, &areas_of_life) {
        Ok(res) => Ok(reply::with_status(reply::json(&res.data), res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::{handle, Request};
    use crate::tests::{app_api, blank_db, response_json_body};
    use cawr_adapter::model::view::json::{thought::create as uc, Error};
    use cawr_application::gateway::repository::thought::Repo;
    use serde_json::Value;
    use warp::{http::StatusCode, Reply};

    #[tokio::test]
    async fn create() {
        let db = blank_db();
        let app_api = app_api(db.clone());
        let req = Request {
            title: "test 1".to_string(),
            areas_of_life: vec![],
        };
        let res = handle(req, app_api).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::CREATED);

        let body: Value = response_json_body(res).await.unwrap();
        let id = body.as_u64().unwrap();
        let record = db.as_ref().get(id.into()).unwrap();

        assert_eq!(record.thought.title().as_ref(), "test 1");
    }

    #[tokio::test]
    async fn create_with_too_short_title() {
        let db = blank_db();
        let app_api = app_api(db);
        let req = Request {
            title: "t".to_string(),
            areas_of_life: vec![],
        };
        let res = handle(req, app_api).await.unwrap().into_response();

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
        let app_api = app_api(db);
        let req = Request {
            title: ["t"; 100].join(""),
            areas_of_life: vec![],
        };
        let res = handle(req, app_api).await.unwrap().into_response();

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
