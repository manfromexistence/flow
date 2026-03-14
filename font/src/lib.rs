//! dx-font - A comprehensive font search and download library
//!
//! This crate provides a production-ready solution for searching and downloading fonts
//! from multiple providers. Access 50k+ commercial-free fonts from 100+ sources including:
//!
//! - Google Fonts (1,562 fonts)
//! - Bunny Fonts (1,478 fonts)
//! - Fontsource (1,562 fonts)
//! - Font Squirrel (1,082 fonts)
//! - DaFont, FontSpace, and many more!
//!
//! ## Features
//!
//! - **Parallel Search**: Blazing fast concurrent search across all providers
//! - **Progress Indication**: Real-time download progress with ETA
//! - **CDN URLs**: Generate CDN URLs for font preview and usage
//! - **Multiple Formats**: Support for TTF, OTF, WOFF, WOFF2
//! - **Caching**: Built-in response caching with configurable TTL
//! - **Rate Limiting**: Automatic rate limiting to prevent API abuse
//! - **Retry Logic**: Exponential backoff for transient failures
//! - **File Verification**: Magic byte and archive validation
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use dx_font::{FontSearch, FontDownloader, FontProvider};
//! use dx_font::models::DownloadOptions;
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> dx_font::FontResult<()> {
//!     // Search for fonts
//!     let search = FontSearch::new()?;
//!     let results = search.search("roboto").await?;
//!     
//!     println!("Found {} fonts", results.total);
//!     for font in results.fonts.iter().take(5) {
//!         println!("  - {} ({})", font.name, font.provider.name());
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Error Handling
//!
//! All operations return [`FontResult<T>`], which is an alias for `Result<T, FontError>`.
//! The [`FontError`] enum provides detailed error information:
//!
//! ```rust,no_run
//! use dx_font::{FontSearch, FontError};
//!
//! #[tokio::main]
//! async fn main() {
//!     let search = FontSearch::new().unwrap();
//!     
//!     match search.search("roboto").await {
//!         Ok(results) => println!("Found {} fonts", results.total),
//!         Err(FontError::Network { url, .. }) => {
//!             eprintln!("Network error accessing {}", url);
//!         }
//!         Err(FontError::AllProvidersFailed { errors }) => {
//!             eprintln!("All providers failed:");
//!             for (provider, error) in errors {
//!                 eprintln!("  - {}: {}", provider, error);
//!             }
//!         }
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! ```
//!
//! ## Configuration
//!
//! Use [`ConfigBuilder`] to customize behavior:
//!
//! ```rust,no_run
//! use dx_font::Config;
//! use std::path::PathBuf;
//!
//! let config = Config::builder()
//!     .output_dir(PathBuf::from("./fonts"))
//!     .timeout_seconds(60)
//!     .max_retries(5)
//!     .build()
//!     .unwrap();
//! ```

/// Cache management for API responses and font metadata.
#[cfg(feature = "full")]
pub mod cache;

/// ZIP extraction utilities
#[cfg(feature = "full")]
pub mod extract;

/// Figlet fonts for ASCII art text rendering in CLI applications.
pub mod figlet;

/// CDN URL generation for font preview and usage.
#[cfg(feature = "full")]
pub mod cdn;

/// Command-line interface implementation.
#[cfg(feature = "full")]
pub mod cli;

/// Configuration and validation.
#[cfg(feature = "full")]
pub mod config;

/// Font download functionality with progress indication.
#[cfg(feature = "full")]
pub mod download;

/// Error types and result aliases.
#[cfg(feature = "full")]
pub mod error;

/// HTTP client with retry logic and rate limiting.
#[cfg(feature = "full")]
pub mod http;

/// Data models for fonts, providers, and search results.
#[cfg(feature = "full")]
pub mod models;

/// Prelude module for convenient imports.
#[cfg(feature = "full")]
pub mod prelude;

/// Font provider implementations.
#[cfg(feature = "full")]
pub mod providers;

/// Rate limiting for API requests.
#[cfg(feature = "full")]
pub mod rate_limit;

/// Font search functionality.
#[cfg(feature = "full")]
pub mod search;

/// File verification for downloaded fonts.
#[cfg(feature = "full")]
pub mod verify;

#[cfg(feature = "full")]
pub use cache::CacheManager;
#[cfg(feature = "full")]
pub use cdn::{CdnProvider, CdnUrlGenerator, FontCdnUrls};
#[cfg(feature = "full")]
pub use config::{Config, ConfigBuilder};
#[cfg(feature = "full")]
pub use download::FontDownloader;
#[cfg(feature = "full")]
pub use error::{FontError, FontResult};
#[cfg(feature = "full")]
pub use http::RetryClient;
#[cfg(feature = "full")]
pub use models::{Font, FontFamily, FontProvider, FontStyle, FontWeight};
#[cfg(feature = "full")]
pub use rate_limit::RateLimiter;
#[cfg(feature = "full")]
pub use search::FontSearch;
#[cfg(feature = "full")]
pub use verify::FileVerifier;
