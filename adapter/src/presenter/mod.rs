pub mod cli;
pub mod json;

pub trait Presenter<D> {
    /// View model
    type ViewModel;
    /// Present the given data `D`
    fn present(&self, data: D) -> Self::ViewModel;
}
