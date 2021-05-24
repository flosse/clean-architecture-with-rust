use crate::id::item::ItemId;
use application::usecase::item::find_by_id::Response;

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
        format!(
            r#"{{"title":"{}","id":"{}"}}"#,
            res.title.to_string(),
            res.id.to_string()
        )
    }
}

impl Presenter for CliPresenter {
    type Out = String;
    fn present(&self, res: Response<ItemId>) -> Self::Out {
        format!("{} ({})", res.title.to_string(), res.id.to_string())
    }
}
