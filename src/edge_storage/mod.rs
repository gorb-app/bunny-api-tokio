//! Edge Storage API
//! 
//! Contains enums, structs and functions for the Bunny Edge Storage API

use crate::error::Error;
use bytes::Bytes;
use reqwest::{header::{HeaderMap, HeaderValue}, Client};
use url::Url;

mod endpoint;
pub use endpoint::Endpoint;
mod list_file;
pub use list_file::ListFile;

/// Edge Storage API for bunny
#[derive(Debug, Clone)]
pub struct EdgeStorageClient {
    pub(crate) url: Url,
    pub(crate) reqwest: Client,
}

impl<'a> EdgeStorageClient {
    /// Creates a new EdgeStorageClient using the supplied `api_key`
    ///
    /// ```
    /// use bunny_api_tokio::{EdgeStorageClient, error::Error, edge_storage::Endpoint};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = EdgeStorageClient::new("storage_zone_api_key", Endpoint::Frankfurt, "MyStorageZone").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn new<T: AsRef<str>, T1: AsRef<str>>(api_key: T, endpoint: Endpoint, storage_zone: T1) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.append("AccessKey", HeaderValue::from_str(api_key.as_ref())?);
        headers.append("accept", HeaderValue::from_str("application/json")?);

        let reqwest = Client::builder().default_headers(headers).build()?;

        let endpoint: Url = endpoint.try_into()?;
        let storage_zone = String::from("/") + storage_zone.as_ref() + "/";

        let url = endpoint.join(&storage_zone)?;

        Ok(Self {
            url,
            reqwest,
        })
    }

    /// Uploads a file to the Storage Zone
    ///
    /// ```
    /// use bunny_api_tokio::{Client, error::Error, edge_storage::Endpoint};
    /// use tokio::fs;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new("api_key").await?;
    ///
    ///     client.storage.init("storage_zone_api_key", Endpoint::Frankfurt, "MyStorageZone").await?;
    ///
    ///     let file_bytes = fs::read("path/to/file.png").await.unwrap();
    ///
    ///     // Will put a file in STORAGE_ZONE/images/file.png
    ///     client.storage.upload("/images/file.png", file_bytes.into()).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn upload<T: AsRef<str>>(&self, path: T, file: Bytes) -> Result<(), Error> {
        let response = self
            .reqwest
            .put(self.url.join(path.as_ref())?)
            .header("Content-Type", "application/octet-stream")
            .body(file)
            .send()
            .await?;

        if response.status().as_u16() == 401 {
            return Err(Error::Authentication(response.text().await?));
        } else if response.status().as_u16() == 400 {
            return Err(Error::BadRequest(response.text().await?));
        }

        Ok(())
    }

    /// Downloads a file from the Storage Zone
    ///
    /// ```
    /// use bunny_api_tokio::{Client, error::Error, edge_storage::Endpoint};
    /// use tokio::fs;
    /// use tokio::io::AsyncWriteExt;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new("api_key").await?;
    ///
    ///     client.storage.init("storage_zone_api_key", Endpoint::Frankfurt, "MyStorageZone").await?;
    ///
    ///     // Will download the file STORAGE_ZONE/images/file.png
    ///     let contents = client.storage.download("/images/file.png").await?;
    ///
    ///     let mut file = fs::File::create("file.png").await.unwrap();
    ///     file.write_all(&contents).await.unwrap();
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn download<T: AsRef<str>>(&self, path: T) -> Result<Bytes, Error> {
        let response = self
            .reqwest
            .get(self.url.join(path.as_ref())?)
            .header("accept", "*/*")
            .send()
            .await?;

        if response.status().as_u16() == 401 {
            return Err(Error::Authentication(response.text().await?));
        } else if response.status().as_u16() == 404 {
            return Err(Error::NotFound(response.text().await?));
        }

        Ok(response.bytes().await?)
    }

    /// Deletes a file from the Storage Zone
    ///
    /// ```
    /// use bunny_api_tokio::{Client, error::Error, edge_storage::Endpoint};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new("api_key").await?;
    ///
    ///     client.storage.init("storage_zone_api_key", Endpoint::Frankfurt, "MyStorageZone").await?;
    ///
    ///     // Will delete the file STORAGE_ZONE/images/file.png
    ///     client.storage.delete("/images/file.png").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete<T: AsRef<str>>(&self, path: T) -> Result<(), Error> {
        let response = self
            .reqwest
            .delete(self.url.join(path.as_ref())?)
            .send()
            .await?;

        if response.status().as_u16() == 401 {
            return Err(Error::Authentication(response.text().await?));
        } else if response.status().as_u16() == 400 {
            return Err(Error::BadRequest(response.text().await?));
        }

        Ok(())
    }

    /// Lists files on the Storage Zone
    ///
    /// ```
    /// use bunny_api_tokio::{Client, error::Error, edge_storage::Endpoint};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new("api_key").await?;
    ///
    ///     client.storage.init("storage_zone_api_key", Endpoint::Frankfurt, "MyStorageZone").await?;
    ///
    ///     // Will list the files in STORAGE_ZONE/images/
    ///     let files = client.storage.list("/images/").await?;
    ///     
    ///     println!("{:#?}", files);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn list<T: AsRef<str>>(&self, path: T) -> Result<Vec<ListFile>, Error> {
        let response = self
            .reqwest
            .get(self.url.join(path.as_ref())?)
            .send()
            .await?;

        if response.status().as_u16() == 401 {
            return Err(Error::Authentication(response.text().await?));
        } else if response.status().as_u16() == 400 {
            return Err(Error::BadRequest(response.text().await?));
        }

        Ok(response.json().await?)
    }
}
