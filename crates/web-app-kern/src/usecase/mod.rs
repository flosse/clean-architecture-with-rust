use std::fmt;

use crate::api;

pub mod area_of_life;
pub mod thought;

// ------ ------
//   Presenter
// ------ ------

trait Present<T> {
    type ViewModel;
    fn present(&self, data: T) -> Self::ViewModel;
}

#[derive(Default)]
struct ErrorPresenter;

impl<E: fmt::Debug> Present<api::Error<E>> for ErrorPresenter {
    type ViewModel = String;
    fn present(&self, err: api::Error<E>) -> Self::ViewModel {
        use gloo_net::Error as F;
        match err {
            api::Error::Fetch(e) => match e {
                F::JsError(_) | F::GlooError(_) => {
                    "A communication problem with the server has occured".to_string()
                }
                F::SerdeError(_) => {
                    "A problem has arisen in the interpretation of the data".to_string()
                }
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
