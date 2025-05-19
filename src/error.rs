//! Error types for this crate.

/// Error type for this crate
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Reqwest error
    #[error("bleh")]
    Reqwest(#[from] reqwest::Error),

    /// Header contains non-ASCII characters
    #[error("header contains non-ASCII characters")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    /// URL Parse error
    #[error("not a valid URL")]
    ParseError(#[from] url::ParseError),

    /// Authentication error
    #[error("authentication error: {0}")]
    Authentication(String),

    /// Bad request error
    #[error("bad request: {0}")]
    BadRequest(String),

    /// Not found error
    #[error("not found: {0}")]
    NotFound(String),
}
