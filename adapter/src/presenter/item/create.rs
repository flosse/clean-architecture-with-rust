use crate::id::item::ItemId;
use application::usecase::item::create::Response;

pub trait Presenter {
    type Out;
    fn present(&self, res: Response<ItemId>) -> Self::Out;
}

#[derive(Default)]
pub struct JsonPresenter;

#[derive(Default)]
pub struct CliPresenter;

impl Presenter for JsonPresenter {
    type Out = String;
    fn present(&self, res: Response<ItemId>) -> Self::Out {
        format!(r#"{{"id":"{}"}}"#, res.id.to_string())
    }
}

impl Presenter for CliPresenter {
    type Out = String;
    fn present(&self, res: Response<ItemId>) -> Self::Out {
        res.id.to_string()
    }
}
