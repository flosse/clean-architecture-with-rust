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
    match api.delete_thought(&req) {
        Ok(res) => Ok(reply_json(&res.data, res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::handle;
    use crate::tests::{add_thought_to_db, app_api, blank_db};
    use cawr_adapter::model::app::thought as app;
    use cawr_application::gateway::repository::thought::Repo;
    use warp::{http::StatusCode, Reply};

    #[tokio::test]
    async fn delete() {
        let db = blank_db();
        add_thought_to_db(&db, "foo");
        add_thought_to_db(&db, "bar");

        let id = "2".parse::<app::Id>().unwrap().into();

        assert!(db.get(id).is_ok());

        let app_api = app_api(db.clone());
        let req = id.to_string();
        let res = handle(req, app_api).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::OK);
        assert!(db.get(id).is_err());
    }
}
