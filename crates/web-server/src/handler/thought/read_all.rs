use crate::{
    handler::{reply_error, Result},
    AppApi,
};
use cawr_adapter::db::Db;
use warp::{reply, Reply};

pub async fn handle<D>(api: AppApi<D>) -> Result<impl Reply>
where
    D: Db,
{
    match api.read_all_thoughts() {
        Ok(res) => Ok(reply::with_status(reply::json(&res.data), res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{add_thought_to_db, app_api, blank_db, response_json_body};
    use serde_json::Value;
    use warp::{http::StatusCode, Reply};

    #[tokio::test]
    async fn read_all() {
        let db = blank_db();
        add_thought_to_db(&db, "foo");
        add_thought_to_db(&db, "bar");

        let api = app_api(db.clone());
        let res = handle(api).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::OK);

        let body: Value = response_json_body(res).await.unwrap();
        let thoughts = body.as_array().unwrap();

        assert_eq!(thoughts.len(), 2);

        let t = thoughts[0].as_object().unwrap();

        assert!(t.get("title").unwrap().is_string());
        assert!(t.get("id").unwrap().is_number());
    }
}
