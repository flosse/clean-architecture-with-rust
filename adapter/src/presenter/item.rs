use domain::usecase::item::create::Response;
use std::fmt::Display;

pub trait Presenter<Id> {
    type Out;
    fn present(&self, res: Response<Id>) -> Self::Out;
}

#[derive(Default)]
pub struct JsonPresenter;

#[derive(Default)]
pub struct CliPresenter;

impl<Id: Display> Presenter<Id> for JsonPresenter {
    type Out = String;
    fn present(&self, res: Response<Id>) -> Self::Out {
        format!(r#"{{"id":"{}"}}"#, res.id)
    }
}

impl<Id: Display> Presenter<Id> for CliPresenter {
    type Out = String;
    fn present(&self, res: Response<Id>) -> Self::Out {
        format!("{}", res.id)
    }
}
