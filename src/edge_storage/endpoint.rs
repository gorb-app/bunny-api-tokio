use url::Url;
use crate::error::Error;

/// Endpoints for Edge Storage API
#[derive(Debug, Clone, PartialEq, Eq)]
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
