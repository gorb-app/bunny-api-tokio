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

use error::Error;
use reqwest::{
    Client as RClient,
    header::{HeaderMap, HeaderValue},
};
use url::Url;

pub mod bunny;
pub mod edge_storage;
pub mod error;

/// API Client for bunny
#[derive(Debug, Clone)]
pub struct Client {
    reqwest: RClient,
    /// Used to interact with the Edge Storage API
    pub storage: edge_storage::Storage,
}

impl Client {
    /// Creates a new Client using the supplied `api_key`
    ///
    /// ```
    /// use bunny_api_tokio::{Client, error::Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Bunny.net api key
    ///     let mut client = Client::new("api_key").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn new<T: AsRef<str>>(api_key: T) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.append("AccessKey", HeaderValue::from_str(api_key.as_ref())?);
        headers.append("accept", HeaderValue::from_str("application/json")?);

        let reqwest = RClient::builder().default_headers(headers).build()?;
        let storage_reqwest = RClient::new();

        Ok(Self {
            reqwest,
            storage: edge_storage::Storage {
                url: Url::parse("https://storage.bunnycdn.com").unwrap(),
                reqwest: storage_reqwest,
            },
        })
    }
}
