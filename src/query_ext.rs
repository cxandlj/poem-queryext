use poem::{http::StatusCode, Error, FromRequest, Result};
use serde::Deserialize;
use std::marker::PhantomData;
use urlencoding::decode;

/// Deserialize struct wrapper.
#[deprecated(
    since = "0.1.2",
    note = "Use `QueryExtN` instead,because `QueryExt` may cause a memory leak"
)]
#[derive(Debug)]
pub struct QueryExt<'a, T>(pub T, PhantomData<&'a T>)
where
    T: Deserialize<'a>;

impl<'a, T> FromRequest<'a> for QueryExt<'a, T>
where
    T: Deserialize<'a>,
{
    async fn from_request(req: &'a poem::Request, _: &mut poem::RequestBody) -> Result<Self> {
        let query = req.uri().query().unwrap_or_default();
        let decode_query = match decode(query) {
            Ok(query) => query.into_owned(),
            Err(err) => {
                return Err(Error::from_string(
                    format!("decode query string error:{}", err),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        };
        //leak the decode_query
        let decode_query = Box::leak(decode_query.into_boxed_str());
        //parse query string to object
        let object = serde_qs::from_str::<T>(decode_query);
        match object {
            Ok(object) => {
                return Ok(QueryExt(object, PhantomData));
            }
            Err(err) => {
                return Err(Error::from_string(
                    format!("parse query string to object error:{}", err),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        }
    }
}

///Deserialize struct new wrapper.
#[derive(Debug)]
pub struct QueryExtN<T>(pub T)
where
    T: for<'a> Deserialize<'a>;

impl<'a, T> FromRequest<'a> for QueryExtN<T>
where
    T: for<'b> Deserialize<'b>,
{
    async fn from_request(req: &'a poem::Request, _: &mut poem::RequestBody) -> Result<Self> {
        let query = req.uri().query().unwrap_or_default();
        let decode_query = match decode(query) {
            Ok(query) => query.into_owned(),
            Err(err) => {
                return Err(Error::from_string(
                    format!("decode query string error:{}", err),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        };
        //parse query string to object
        let object = serde_qs::from_str::<T>(&decode_query);
        match object {
            Ok(object) => {
                return Ok(Self(object));
            }
            Err(err) => {
                return Err(Error::from_string(
                    format!("parse query string to object error:{}", err),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        }
    }
}
