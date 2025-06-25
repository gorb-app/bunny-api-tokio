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

#[cfg(feature = "bunnynet")]
mod bunny;
#[cfg(feature = "bunnynet")]
pub use bunny::BunnyClient;
#[cfg(feature = "edge_storage")]
pub mod edge_storage;
#[cfg(feature = "edge_storage")]
pub use edge_storage::EdgeStorageClient;
pub mod error;
