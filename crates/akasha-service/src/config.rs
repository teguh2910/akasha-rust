//! Service configuration loaded from the environment.

use std::time::Duration;

use akasha::BASE_URL;

/// Port used when `PORT` is not set.
pub const DEFAULT_PORT: u16 = 8099;

/// Default lifetime of a cached upstream response.
pub const DEFAULT_CACHE_TTL: Duration = Duration::from_secs(360);

/// Runtime configuration for the service.
#[derive(Debug, Clone)]
pub struct Config {
    /// Port to listen on.
    pub port: u16,
    /// Root of the Akasha API, including the `/api` segment.
    pub base_url: String,
    /// How long the Akasha client keeps responses in memory.
    pub cache_ttl: Duration,
}

/// Why the configuration could not be loaded.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// A variable was present but unusable.
    #[error("invalid {name}: {reason}")]
    Invalid {
        /// Variable name.
        name: &'static str,
        /// What is wrong with it.
        reason: String,
    },
}

impl Config {
    /// Reads configuration from the process environment.
    pub fn from_env() -> Result<Self, ConfigError> {
        let port = match std::env::var("PORT") {
            Ok(raw) => raw.parse().map_err(|_| ConfigError::Invalid {
                name: "PORT",
                reason: format!("expected a port number, got `{raw}`"),
            })?,
            Err(_) => DEFAULT_PORT,
        };

        let base_url = std::env::var("AKASHA_BASE_URL").unwrap_or_else(|_| BASE_URL.to_owned());
        if !(base_url.starts_with("http://") || base_url.starts_with("https://")) {
            return Err(ConfigError::Invalid {
                name: "AKASHA_BASE_URL",
                reason: format!("expected an http(s) URL, got `{base_url}`"),
            });
        }

        let cache_ttl = match std::env::var("CACHE_TTL_SECS") {
            Ok(raw) => {
                let secs: u64 = raw.parse().map_err(|_| ConfigError::Invalid {
                    name: "CACHE_TTL_SECS",
                    reason: format!("expected a whole number of seconds, got `{raw}`"),
                })?;
                Duration::from_secs(secs)
            }
            Err(_) => DEFAULT_CACHE_TTL,
        };

        Ok(Self {
            port,
            base_url,
            cache_ttl,
        })
    }
}
