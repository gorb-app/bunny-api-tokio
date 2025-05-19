//! This library provides access to the Bunny API asynchronously using tokio, it's not fully implemented but PRs are welcome.
//! 
//! # Getting started
//! 1. add package to your project using cargo
//! 
//! `$ cargo add bunny-api-tokio`
//! 
//! 2. Start coding
//! 
//! ```
//! use bunny_api_tokio::{Client, error::Error};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let mut client = Client::new("api_key").await?;
//! 
//!     Ok(())
//! }
//! ```
#![deny(missing_docs)]

use std::sync::Arc;
use reqwest::{header::{HeaderMap, HeaderValue}, Client as RClient};
use error::Error;
use url::Url;

pub mod error;
pub mod edge_storage;

/// API Client for bunny
pub struct Client {
    /// Used to interact with the Edge Storage API
    pub storage: edge_storage::Storage
}

impl Client {
    /// Creates a new Client using the supplied `api_key`
    /// 
    /// ```
    /// use bunny_api_tokio::{Client, error::Error};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new("api_key").await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub async fn new<T: AsRef<str>>(api_key: T) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.append("AccessKey", HeaderValue::from_str(api_key.as_ref())?);

        let reqwest = Arc::new(RClient::builder()
            .default_headers(headers)
            .build()?);

        Ok(Self {
            storage: edge_storage::Storage {
                url: Url::parse("https://storage.bunnycdn.com").unwrap(),
                reqwest,
            },
        })
    }
}
