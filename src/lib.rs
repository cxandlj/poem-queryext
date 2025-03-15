#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(clippy::dbg_macro, clippy::use_debug)]
#![warn(missing_docs, missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(deprecated)]

//! Implement poem FromRequest to deserialize struct from query string.
//!
//! #### Example
//! ```no_run
//! use poem_queryext::QueryExtN;
//! use poem_openapi::{payload::PlainText, OpenApi};
//! use serde::Deserialize;
//!
//! struct Api;
//!
//! #[OpenApi]
//! impl Api {
//!   //test url: /test?name=cx  
//!   //test url: /test?name=cx&age=18&hobby[0]=music&hobby[1]=game  
//!   #[oai(path = "/test", method = "get")]
//!   async fn test(&self, QueryExtN(query): QueryExtN<QueryObj>) -> PlainText<String> {
//!        PlainText(format!(
//!            "name:{},age:{},hobby:{}",
//!            query.name,
//!            query.age.unwrap_or_default(),
//!            query.hobby.unwrap_or_default().join(",")
//!        ))
//!    }
//! }
//!
//! #[derive(Deserialize)]
//! #[serde(rename_all = "camelCase")]
//! struct QueryObj{
//!     name:String,//if want use &str,use Arc<str> or Cow<'_,str>
//!     age:Option<i8>,//Non mandatory fields  use Option<T>
//!     hobby:Option<Vec<String>>//Non mandatory fields  use Option<T>
//! }
//!
//! ```
mod query_ext;

pub use query_ext::QueryExt;
pub use query_ext::QueryExtN;
