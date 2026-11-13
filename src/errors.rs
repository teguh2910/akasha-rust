//! Error types returned by the API client.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// An error raised by the Akasha API or the client itself.
#[derive(Debug, Error)]
pub enum Error {
    /// A transport level error.
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    /// A JSON (de)serialization error.
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    /// A URL could not be parsed.
    #[error("invalid URL: {0}")]
    Url(String),

    /// A header name or value was invalid.
    #[error("invalid header: {0}")]
    Header(String),

    /// Base class for all Akasha API errors.
    #[error("{title}: {description}")]
    Api { title: String, description: String },

    /// Raised when the provided UID cannot be fetched from Enka Network API.
    #[error("{title}: {description}")]
    UidNotFound { title: String, description: String },

    /// Raised when the provided UID is not in the correct format.
    #[error("{title}: {description}")]
    IncorrectUidFormat { title: String, description: String },

    /// Raised when the API request is invalid.
    #[error("{title}: {description}")]
    InvalidApiRequest { title: String, description: String },

    /// A field expected in the response was missing.
    #[error("missing field: {0}")]
    MissingField(&'static str),

    /// A translation for a word was not returned by the API.
    #[error("translation not found for {0:?}")]
    MissingTranslation(String),

    /// An enum variant did not match any known value.
    #[error("unknown variant: {0}")]
    UnknownVariant(String),

    /// The requested functionality is not implemented.
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
}

/// Map an API error description to its dedicated error variant.
pub(crate) fn error_from_description(title: String, description: String) -> Error {
    match description.as_str() {
        "Provided UID cannot be fetched from Enka Network API" => {
            Error::UidNotFound { title, description }
        }
        "The UID format is incorrect" => Error::IncorrectUidFormat { title, description },
        _ => Error::Api { title, description },
    }
}
