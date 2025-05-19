//! This library provides access to the Bunny API asynchronously using tokio, it's not fully implemented but PRs are welcome.
#![deny(missing_docs)]

use std::{sync::Arc};
use bytes::Bytes;
use log::debug;
use reqwest::{header::{HeaderMap, HeaderValue}, Client as RClient};
use error::Error;
use url::Url;
pub use reqwest::multipart;

pub mod error;

/// Endpoints for Edge Storage API
pub enum StorageEndpoint {
    /// Uses https://storage.bunnycdn.com as endpoint
    Frankfurt,
    /// Uses https://uk.storage.bunnycdn.com as endpoint
    London,
    /// Uses https://ny.storage.bunnycdn.com as endpoint
    NewYork,
    /// Uses https://la.storage.bunnycdn.com as endpoint
    LosAngeles,
    /// Uses https://sg.storage.bunnycdn.com as endpoint
    Singapore,
    /// Uses https://se.storage.bunnycdn.com as endpoint
    Stockholm,
    /// Uses https://br.storage.bunnycdn.com as endpoint
    SaoPaulo,
    /// Uses https://jh.storage.bunnycdn.com as endpoint
    Johannesburg,
    /// Uses https://syd.storage.bunnycdn.com as endpoint
    Sydney,
    /// Lets you input a custom endpoint, in case bunny adds a new one and this crate isnt up-to-date, has to be a valid URL with http(s) in front
    Custom(String),
}

impl TryInto<Url> for StorageEndpoint {
    type Error = Error;

    fn try_into(self) -> Result<Url, Error> {
        match self {
            StorageEndpoint::Frankfurt => Ok(Url::parse("https://storage.bunnycdn.com")?),
            StorageEndpoint::London => Ok(Url::parse("https://uk.storage.bunnycdn.com")?),
            StorageEndpoint::NewYork => Ok(Url::parse("https://ny.storage.bunnycdn.com")?),
            StorageEndpoint::LosAngeles => Ok(Url::parse("https://la.storage.bunnycdn.com")?),
            StorageEndpoint::Singapore => Ok(Url::parse("https://sg.storage.bunnycdn.com")?),
            StorageEndpoint::Stockholm => Ok(Url::parse("https://se.storage.bunnycdn.com")?),
            StorageEndpoint::SaoPaulo => Ok(Url::parse("https://br.storage.bunnycdn.com")?),
            StorageEndpoint::Johannesburg => Ok(Url::parse("https://jh.storage.bunnycdn.com")?),
            StorageEndpoint::Sydney => Ok(Url::parse("https://syd.storage.bunnycdn.com")?),
            StorageEndpoint::Custom(url) => Ok(Url::parse(&url)?),
        }
    }
}

/// Edge Storage API for bunny
pub struct Storage {
    url: Url,
    reqwest: Arc<RClient>,
}

impl<'a> Storage {
    /// Sets endpoint and storage zone used by Edge Storage API
    /// 
    /// ```
    /// use bunny_api_tokio::{Client, error::Error, StorageEndpoint};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new("api_key")?;
    /// 
    ///     client.storage.init(StorageEndpoint::Frankfurt, "MyStorageZone");
    /// }
    /// ```
    pub fn init<T: AsRef<str>>(&mut self, endpoint: StorageEndpoint, storage_zone: T) -> Result<(), Error> {
        let endpoint: Url = endpoint.try_into()?;
        let storage_zone = String::from("/") + storage_zone.as_ref() + "/";

        self.url = endpoint.join(&storage_zone)?;
        Ok(())
    }

    /// Uploads a file to the Storage Zone
    /// 
    /// ```
    /// use bunny_api_tokio::{Client, error::Error, StorageEndpoint};
    /// use tokio::fs;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new("api_key")?;
    /// 
    ///     client.storage.init(StorageEndpoint::Frankfurt, "MyStorageZone");
    /// 
    ///     let file_bytes = fs::read("path/to/file.png").await?;
    /// 
    ///     // Will put a file in STORAGE_ZONE/images/file.png
    ///     client.storage.upload("/images/file.png", file_bytes);
    /// }
    /// ```
    pub async fn upload<T: AsRef<str>>(&self, path: T, file: Bytes) -> Result<(), Error> {
        let response  = self.reqwest.put(self.url.join(path.as_ref())?)
            .header("Content-Type", "application/octet-stream")
            .body(file)
            .send()
            .await?
            .text()
            .await?;

        debug!("{}", response);

        Ok(())
    }
}

/// API Client for bunny
pub struct Client {
    reqwest: Arc<RClient>,
    /// Used to interact with the Edge Storage API
    pub storage: Storage
}

impl Client {
    /// Creates a new Client using the supplied `api_key`
    /// 
    /// ```
    /// use bunny_api_tokio::{Client, error::Error};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new("api_key")?;
    /// }
    /// ```
    pub async fn new<T: AsRef<str>>(api_key: T) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.append("AccessKey", HeaderValue::from_str(api_key.as_ref())?);

        let reqwest = Arc::new(RClient::builder()
            .default_headers(headers)
            .build()?);

        Ok(Self {
            reqwest: reqwest.clone(),
            storage: Storage {
                url: Url::parse("https://storage.bunnycdn.com").unwrap(),
                reqwest,
            },
        })
    }
}
