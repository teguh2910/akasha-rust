//! HTTP service exposing the Akasha System API.
//!
//! The service wraps the [`akasha`] client in a small JSON API so browsers and
//! other programs can read akasha.cv data without embedding the Rust crate.

pub mod config;
pub mod error;
pub mod routes;
pub mod state;

pub use config::{Config, ConfigError};
pub use error::ApiError;
pub use state::AppState;
