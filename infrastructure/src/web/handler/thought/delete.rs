use crate::web::handler::{reply_error, Result};
use adapter::{
    controller::thought::delete::Controller, model::app::thought::Id,
    presenter::http_json_api::Presenter,
};
use application::gateway::repository::thought::Repo;
use std::sync::Arc;
use warp::{reply, Reply};

pub type Request = String;

pub async fn handle<R>(req: Request, repo: Arc<R>) -> Result<impl Reply>
where
    R: Repo<Id = Id> + 'static,
{
    let presenter = Presenter::default();
    let controller = Controller::new(repo, presenter);
    let res = controller.delete_thought(&req);
    match res {
        Ok(res) => Ok(reply::with_status(reply::json(&res.data), res.status)),
        Err(err) => Ok(reply_error(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::{handle, Repo};
    use crate::web::tests::{add_thought_to_db, blank_db};
    use warp::{http::StatusCode, Reply};

    #[tokio::test]
    async fn delete() {
        let db = blank_db();
        add_thought_to_db(&db, "foo");
        add_thought_to_db(&db, "bar");

        let id = "2".parse().unwrap();

        assert!(db.get(id).is_ok());

        let req = id.to_string();
        let res = handle(req, db.clone()).await.unwrap().into_response();

        assert_eq!(res.status(), StatusCode::OK);
        assert!(db.get(id).is_err());
    }
}
