//! Contains structs, enums and implementations for the main bunny.net API

use serde::Deserialize;
use url::Url;

use crate::{Client, error::Error};

/// Country struct returned by get_countries() function
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Country {
    /// Country name
    pub name: String,
    /// Country ISO code
    pub iso_code: String,
    /// Country is part of the EU
    #[serde(rename = "IsEU")]
    pub is_eu: bool,
    /// Tax rate in percentage
    pub tax_rate: f32,
    /// Tax prefix
    pub tax_prefix: String,
    /// URL to country flag
    pub flag_url: Url,
    /// ??
    pub pop_list: Vec<String>,
}

/// API Key struct returned by list_api_keys()
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ApiKey {
    /// API Key ID
    pub id: i32,
    /// API Key
    pub key: String,
    /// ??
    pub roles: Vec<String>,
}

/// Pagination struct used by Bunny.net API
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Pagination<T> {
    /// Vector of type T
    pub items: Vec<T>,
    /// Current page number
    pub current_page: i32,
    /// Total amount of type T
    pub total_items: i32,
    /// Has more items
    pub has_more_items: bool,
}

/// Region struct returned by region_list()
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Region {
    /// Region ID
    pub id: i32,
    /// Name of the region
    pub name: String,
    /// Price per gigabyte in region
    pub price_per_gigabyte: f32,
    /// Region 2 letter code
    pub region_code: String,
    /// Continent 2 letter code
    pub continent_code: String,
    /// Country 2 letter code
    pub country_code: String,
    /// Region latitude
    pub latitude: f32,
    /// Region longitude
    pub longitude: f32,
    /// ??
    pub allow_latency_routing: bool,
}

impl Client {
    // TODO: Following functions could probably use better naming, the names are currently derived from the titles on the API reference

    /// Returns a list of countries and tax rates
    ///
    /// ```
    /// use bunny_api_tokio::{Client, error::Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Bunny.net api key
    ///     let mut client = Client::new("api_key").await?;
    ///
    ///     let countries = client.get_countries().await?;
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
    /// use bunny_api_tokio::{Client, error::Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Bunny.net api key
    ///     let mut client = Client::new("api_key").await?;
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
    /// use bunny_api_tokio::{Client, error::Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Bunny.net api key
    ///     let mut client = Client::new("api_key").await?;
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
    /// use bunny_api_tokio::{Client, error::Error};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Error> {
    ///     // Bunny.net api key
    ///     let mut client = Client::new("api_key").await?;
    ///
    ///     client.purge_url("https://url_to_purge.com", false).await?;
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
