pub mod cli;
pub mod http_json_api;

/// The Presenter
///
/// *"Its job is to accept data from the application
/// and format it for presentation so that the **view**
/// can simply move it to the screen"* [^ca-presenter].
///
/// [^ca-presenter]: Robert C. Martin, Clean Architecture, 2017, p. 203.
pub trait Present<D> {
    /// View model
    type ViewModel;
    /// Present the given data `D`
    fn present(&self, data: D) -> Self::ViewModel;
}
