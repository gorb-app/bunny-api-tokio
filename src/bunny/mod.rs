//! Contains structs, enums and implementations for the main Bunny.net API

use url::Url;

use crate::error::Error;
use reqwest::{
    Client as RClient,
    header::{HeaderMap, HeaderValue},
};

mod api_key;
pub use api_key::ApiKey;
mod country;
pub use country::Country;
mod pagination;
pub use pagination::Pagination;
mod region;
pub use region::Region;

/// API Client for bunny.net
#[derive(Debug, Clone)]
pub struct BunnyClient {
    reqwest: RClient,
}

impl BunnyClient {
    /// Creates a new Bunny.net API Client using the supplied `api_key`
    ///
    /// ```
    /// use bunny_api_tokio::{BunnyClient, error::Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Bunny.net api key
    ///     let mut client = BunnyClient::new("api_key").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn new<T: AsRef<str>>(api_key: T) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.append("AccessKey", HeaderValue::from_str(api_key.as_ref())?);
        headers.append("accept", HeaderValue::from_str("application/json")?);

        let reqwest = RClient::builder().default_headers(headers).build()?;

        Ok(Self {
            reqwest,
        })
    }

    // TODO: Following functions could probably use better naming, the names are currently derived from the titles on the API reference

    /// Returns a list of countries and tax rates
    ///
    /// ```
    /// use bunny_api_tokio::{BunnyClient, error::Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Bunny.net api key
    ///     let mut client = BunnyClient::new("api_key").await?;
    ///
    ///     let countries = client.get_country_list().await?;
    ///
    ///     println!("{:#?}", countries);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_country_list(&self) -> Result<Vec<Country>, Error> {
        let response = self
            .reqwest
            .get("https://api.bunny.net/country")
            .header("accept", "application/json")
            .send()
            .await?;

        if response.status().as_u16() == 401 {
            return Err(Error::Authentication(response.text().await?));
        } else if response.status().as_u16() == 500 {
            return Err(Error::InternalServerError(response.text().await?));
        }

        Ok(response.json().await?)
    }

    /// Returns a list of API Keys
    ///
    /// ```
    /// use bunny_api_tokio::{BunnyClient, error::Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Bunny.net api key
    ///     let mut client = BunnyClient::new("api_key").await?;
    ///
    ///     let api_keys = client.list_api_keys(1, 1000).await?;
    ///
    ///     println!("{:#?}", api_keys);
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_api_keys(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Pagination<ApiKey>, Error> {
        let response = self
            .reqwest
            .get("https://api.bunny.net/apikey")
            .query(&[("page", page), ("perPage", per_page)])
            .send()
            .await?;

        if response.status().as_u16() == 401 {
            return Err(Error::Authentication(response.text().await?));
        } else if response.status().as_u16() == 500 {
            return Err(Error::InternalServerError(response.text().await?));
        }

        Ok(response.json().await?)
    }

    /// Returns a list of Regions
    ///
    /// ```
    /// use bunny_api_tokio::{BunnyClient, error::Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Bunny.net api key
    ///     let mut client = BunnyClient::new("api_key").await?;
    ///
    ///     let regions = client.region_list().await?;
    ///
    ///     println!("{:#?}", regions);
    ///     Ok(())
    /// }
    /// ```
    pub async fn region_list(&self) -> Result<Vec<Region>, Error> {
        let response = self
            .reqwest
            .get("https://api.bunny.net/region")
            .send()
            .await?;

        if response.status().as_u16() == 401 {
            return Err(Error::Authentication(response.text().await?));
        } else if response.status().as_u16() == 500 {
            return Err(Error::InternalServerError(response.text().await?));
        }

        Ok(response.json().await?)
    }

    /// Purges a URL from the cache
    ///
    /// ```
    /// use bunny_api_tokio::{BunnyClient, error::Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Bunny.net api key
    ///     let mut client = BunnyClient::new("api_key").await?;
    ///
    ///     client.purge_url("https://url_to_purge.com".parse()?, false).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn purge_url(&self, url: Url, asynchronous: bool) -> Result<(), Error> {
        let response = self
            .reqwest
            .post("https://api.bunny.net/purge")
            .query(&[
                ("url", url.to_string()),
                ("async", asynchronous.to_string()),
            ])
            .send()
            .await?;

        if response.status().as_u16() == 401 {
            return Err(Error::Authentication(response.text().await?));
        } else if response.status().as_u16() == 500 {
            return Err(Error::InternalServerError(response.text().await?));
        }

        Ok(response.json().await?)
    }
}
