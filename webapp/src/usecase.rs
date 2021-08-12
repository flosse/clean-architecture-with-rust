pub mod thought {

    use crate::{
        api,
        domain::{Thought, ThoughtId},
    };

    // ------ ------
    //  Controller
    // ------ ------

    pub async fn create(title: String) -> Result<ThoughtId, String> {
        let presenter = ErrorPresenter::default();
        api::create_thought(title)
            .await
            .map_err(|e| presenter.present(e))
    }

    pub async fn find_by_id(id: &ThoughtId) -> Result<Thought, String> {
        let presenter = ErrorPresenter::default();
        api::fetch_thought(id)
            .await
            .map_err(|e| presenter.present(e))
    }

    pub async fn fetch_all() -> Result<Vec<Thought>, String> {
        let presenter = ErrorPresenter::default();
        api::fetch_all_thoughts()
            .await
            .map_err(|e| presenter.present(e))
    }

    pub async fn delete(id: &ThoughtId) -> Result<(), String> {
        let presenter = ErrorPresenter::default();
        api::delete_thought(id)
            .await
            .map_err(|e| presenter.present(e))
    }

    // ------ ------
    //   Presenter
    // ------ ------

    trait Present<T> {
        type ViewModel;
        fn present(&self, data: T) -> Self::ViewModel;
    }

    #[derive(Default)]
    struct ErrorPresenter;

    impl<E: std::fmt::Debug> Present<api::Error<E>> for ErrorPresenter {
        type ViewModel = String;
        fn present(&self, err: api::Error<E>) -> Self::ViewModel {
            use seed::fetch::FetchError as F;

            match err {
                api::Error::Fetch(e) => match e {
                    F::RequestError(_)
                    | F::StatusError(_)
                    | F::PromiseError(_)
                    | F::SerdeError(_) => {
                        "A communication problem with the server has occured".to_string()
                    }
                    F::DomException(e) => {
                        format!("A problem within the browser has occured: {:?}", e)
                    }
                    F::NetworkError(_) => "A network error has occured".to_string(),
                },
                api::Error::Api(e) => {
                    if let Some(d) = &e.details {
                        format!("{:?}", d) // TODO
                    } else if let Some(m) = &e.msg {
                        m.to_string()
                    } else {
                        format!("{:?}", e) // TODO
                    }
                }
            }
        }
    }
}
