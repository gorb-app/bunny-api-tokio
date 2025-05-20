//! B

use std::sync::Arc;

use crate::Error;
use bytes::Bytes;
use reqwest::Client;
use serde::Deserialize;
use url::Url;

/// Endpoints for Edge Storage API
pub enum Endpoint {
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

impl TryInto<Url> for Endpoint {
    type Error = Error;

    fn try_into(self) -> Result<Url, Error> {
        match self {
            Endpoint::Frankfurt => Ok(Url::parse("https://storage.bunnycdn.com")?),
            Endpoint::London => Ok(Url::parse("https://uk.storage.bunnycdn.com")?),
            Endpoint::NewYork => Ok(Url::parse("https://ny.storage.bunnycdn.com")?),
            Endpoint::LosAngeles => Ok(Url::parse("https://la.storage.bunnycdn.com")?),
            Endpoint::Singapore => Ok(Url::parse("https://sg.storage.bunnycdn.com")?),
            Endpoint::Stockholm => Ok(Url::parse("https://se.storage.bunnycdn.com")?),
            Endpoint::SaoPaulo => Ok(Url::parse("https://br.storage.bunnycdn.com")?),
            Endpoint::Johannesburg => Ok(Url::parse("https://jh.storage.bunnycdn.com")?),
            Endpoint::Sydney => Ok(Url::parse("https://syd.storage.bunnycdn.com")?),
            Endpoint::Custom(url) => Ok(Url::parse(&url)?),
        }
    }
}

/// File information returned by list
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ListFile {
    /// ??
    pub guid: String,
    /// Name of the storage zone the object is in
    pub storage_zone_name: String,
    /// Path to object
    pub path: String,
    /// Object name
    pub object_name: String,
    /// Length of the object in bytes
    pub length: u32,
    /// When the object was last modified
    pub last_changed: String,
    /// ??
    pub server_id: u32,
    /// ??
    pub array_number: u32,
    /// If the object is a directory
    pub is_directory: bool,
    /// ??
    pub user_id: String,
    /// Object content type
    pub content_type: String,
    /// When the object was created
    pub date_created: String,
    /// ID of the storage zone the object is in
    pub storage_zone_id: u32,
    /// File checksum on server
    pub checksum: String,
    /// Zones the object is replicated to
    pub replicated_zones: String,
}

/// Edge Storage API for bunny
pub struct Storage {
    pub(crate) url: Url,
    pub(crate) reqwest: Arc<Client>,
}

impl<'a> Storage {
    /// Sets endpoint and storage zone used by Edge Storage API
    ///
    /// ```
    /// use bunny_api_tokio::{Client, error::Error, edge_storage::Endpoint};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     let mut client = Client::new("api_key").await?;
    ///
    ///     client.storage.init(Endpoint::Frankfurt, "MyStorageZone");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn init<T: AsRef<str>>(
        &mut self,
        endpoint: Endpoint,
        storage_zone: T,
    ) -> Result<(), Error> {
        let endpoint: Url = endpoint.try_into()?;
        let storage_zone = String::from("/") + storage_zone.as_ref() + "/";

        self.url = endpoint.join(&storage_zone)?;
        Ok(())
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
    ///     client.storage.init(Endpoint::Frankfurt, "MyStorageZone");
    ///
    ///     let file_bytes = fs::read("path/to/file.png").await?;
    ///
    ///     // Will put a file in STORAGE_ZONE/images/file.png
    ///     client.storage.upload("/images/file.png", file_bytes).await?;
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
    ///     client.storage.init(Endpoint::Frankfurt, "MyStorageZone");
    ///
    ///     // Will download the file STORAGE_ZONE/images/file.png
    ///     let contents = client.storage.download("/images/file.png").await?;
    ///
    ///     let mut file = fs::File::create("file.png").await?;
    ///     file.write_all(contents).await?;
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
    ///     client.storage.init(Endpoint::Frankfurt, "MyStorageZone");
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
    ///     client.storage.init(Endpoint::Frankfurt, "MyStorageZone");
    ///
    ///     // Will list the files in STORAGE_ZONE/images/
    ///     let files = client.storage.list("/images/").await?;
    ///     
    ///     println!("{:#?}", files)
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
