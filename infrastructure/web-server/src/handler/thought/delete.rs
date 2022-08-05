use crate::handler::{reply_error, Result};
use adapter::{controller::thought::Controller, db::Db, presenter::http_json_api::Presenter};
use std::sync::Arc;
use warp::{reply, Reply};

pub type Request = String;

pub async fn handle<D>(
    req: Request,
    controller: Arc<Controller<D, Presenter>>,
) -> Result<impl Reply>
where
    D: Db,
{
    match controller.delete_thought(&req) {
        Ok(res) => Ok(reply::with_status(reply::json(&res.data), res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::{handle, Arc, Controller, Presenter};
    use crate::tests::{add_thought_to_db, blank_db};
    use adapter::model::app::thought as app;
    use application::gateway::repository::thought::Repo;
    use warp::{http::StatusCode, Reply};

    #[tokio::test]
    async fn delete() {
        let db = blank_db();
        add_thought_to_db(&db, "foo");
        add_thought_to_db(&db, "bar");

        let id = "2".parse::<app::Id>().unwrap().into();

        assert!(db.get(id).is_ok());

        let controller = Arc::new(Controller::new(db.clone(), Presenter::default()));
        let req = id.to_string();
        let res = handle(req, controller).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::OK);
        assert!(db.get(id).is_err());
    }
}
