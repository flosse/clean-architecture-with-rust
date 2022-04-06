use json_boundary as boundary;
use seed::browser::fetch::{fetch, FetchError, Method, Request, Response};
use serde::{Deserialize, Serialize};
use std::result;

#[derive(Debug)]
pub enum Error<E> {
    Fetch(FetchError),
    Api(boundary::Error<E>),
}

impl<E> From<FetchError> for Error<E> {
    fn from(e: FetchError) -> Self {
        Self::Fetch(e)
    }
}

pub type Result<T, E> = result::Result<T, Error<E>>;

pub async fn get_json<T, E>(url: &str) -> Result<T, E>
where
    T: for<'de> Deserialize<'de> + 'static,
    E: for<'de> Deserialize<'de> + 'static,
{
    let res = fetch(url).await?;
    to_result(res).await
}

pub async fn post_json<R, T, E>(url: &str, req: &R) -> Result<T, E>
where
    R: Serialize,
    T: for<'de> Deserialize<'de> + 'static,
    E: for<'de> Deserialize<'de> + 'static,
{
    let req = Request::new(url).method(Method::Post).json(req)?;
    let res = fetch(req).await?;
    to_result(res).await
}

pub async fn put_json<R, T, E>(url: &str, req: &R) -> Result<T, E>
where
    R: Serialize,
    T: for<'de> Deserialize<'de> + 'static,
    E: for<'de> Deserialize<'de> + 'static,
{
    let req = Request::new(url).method(Method::Put).json(req)?;
    let res = fetch(req).await?;
    to_result(res).await
}

pub async fn delete_json<R, T, E>(url: &str, req: &R) -> Result<T, E>
where
    R: Serialize,
    T: for<'de> Deserialize<'de> + 'static,
    E: for<'de> Deserialize<'de> + 'static,
{
    let req = Request::new(url).method(Method::Delete).json(req)?;
    let res = fetch(req).await?;
    to_result(res).await
}

async fn to_result<T, E>(res: Response) -> Result<T, E>
where
    T: for<'de> Deserialize<'de> + 'static,
    E: for<'de> Deserialize<'de> + 'static,
{
    if res.status().is_ok() {
        let data = res.json().await?;
        Ok(data)
    } else {
        let error = res.json().await?;
        Err(Error::Api(error))
    }
}
