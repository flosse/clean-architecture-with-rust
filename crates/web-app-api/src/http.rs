use std::result;

use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use cawr_json_boundary as boundary;

#[derive(Debug, Error)]
pub enum Error<E> {
    Fetch(#[from] gloo_net::Error),
    Api(boundary::Error<E>),
}

pub(crate) type Result<T, E> = result::Result<T, Error<E>>;

pub async fn get_json<T, E>(url: &str) -> Result<T, E>
where
    T: for<'de> Deserialize<'de> + 'static,
    E: for<'de> Deserialize<'de> + 'static,
{
    let res = Request::get(url).send().await?;
    to_result(res).await
}

pub async fn post_json<R, T, E>(url: &str, req: &R) -> Result<T, E>
where
    R: Serialize,
    T: for<'de> Deserialize<'de> + 'static,
    E: for<'de> Deserialize<'de> + 'static,
{
    let req = Request::post(url).json(req)?;
    let res = req.send().await?;
    to_result(res).await
}

pub async fn put_json<R, T, E>(url: &str, req: &R) -> Result<T, E>
where
    R: Serialize,
    T: for<'de> Deserialize<'de> + 'static,
    E: for<'de> Deserialize<'de> + 'static,
{
    let req = Request::put(url).json(req)?;
    let res = req.send().await?;
    to_result(res).await
}

pub async fn delete_json<R, T, E>(url: &str, req: &R) -> Result<T, E>
where
    R: Serialize,
    T: for<'de> Deserialize<'de> + 'static,
    E: for<'de> Deserialize<'de> + 'static,
{
    let req = Request::delete(url).json(req)?;
    let res = req.send().await?;
    to_result(res).await
}

async fn to_result<T, E>(res: Response) -> Result<T, E>
where
    T: for<'de> Deserialize<'de> + 'static,
    E: for<'de> Deserialize<'de> + 'static,
{
    if res.ok() {
        let data = res.json().await?;
        Ok(data)
    } else {
        let error = res.json().await?;
        Err(Error::Api(error))
    }
}
