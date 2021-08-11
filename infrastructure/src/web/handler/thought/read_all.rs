use crate::web::handler::{reply_error, Result};
use adapter::{
    controller::thought::read_all::Controller, model::app::thought::Id,
    presenter::http_json_api::Presenter,
};
use application::gateway::repository::thought::Repo;
use std::sync::Arc;
use warp::{reply, Reply};

pub async fn handle<R>(repo: Arc<R>) -> Result<impl Reply>
where
    R: Repo<Id = Id> + 'static,
{
    let presenter = Presenter::default();
    let controller = Controller::new(repo, presenter);
    let res = controller.read_all();
    match res {
        Ok(res) => Ok(reply::with_status(reply::json(&res.data), res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::handle;
    use crate::web::tests::{add_thought_to_db, blank_db, response_json_body};
    use serde_json::Value;
    use warp::{http::StatusCode, Reply};

    #[tokio::test]
    async fn read_all() {
        let db = blank_db();
        add_thought_to_db(&db, "foo");
        add_thought_to_db(&db, "bar");

        let res = handle(db.clone()).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::OK);

        let body: Value = response_json_body(res).await.unwrap();
        let thoughts = body.as_array().unwrap();

        assert_eq!(thoughts.len(), 2);

        let t = thoughts[0].as_object().unwrap();

        assert!(t.get("title").unwrap().is_string());
        assert!(t.get("id").unwrap().is_string());
    }
}
