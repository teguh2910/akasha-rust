//! Maps failures onto JSON HTTP responses.

use akasha::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

/// An error rendered as a JSON HTTP response.
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    body: serde_json::Value,
}

impl ApiError {
    fn new(status: StatusCode, body: serde_json::Value) -> Self {
        Self { status, body }
    }

    /// The request parameters were not valid.
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            json!({ "error": "bad_request", "message": message.into() }),
        )
    }

    /// Translates an upstream or client error.
    ///
    /// Akasha failures are reported as `5xx` unless the caller sent bad input,
    /// because a blocked or unreachable upstream is not their mistake.
    pub fn from_akasha(error: Error) -> Self {
        match error {
            Error::UidNotFound { title, description } => Self::new(
                StatusCode::NOT_FOUND,
                json!({ "error": "uid_not_found", "title": title, "message": description }),
            ),
            Error::IncorrectUidFormat { title, description } => Self::new(
                StatusCode::BAD_REQUEST,
                json!({ "error": "invalid_uid", "title": title, "message": description }),
            ),
            Error::InvalidApiRequest { title, description } => Self::new(
                StatusCode::BAD_REQUEST,
                json!({ "error": "invalid_request", "title": title, "message": description }),
            ),
            Error::Http(error) if error.is_timeout() => Self::new(
                StatusCode::GATEWAY_TIMEOUT,
                json!({ "error": "upstream_timeout", "message": error.to_string() }),
            ),
            Error::Http(error) => Self::new(
                StatusCode::BAD_GATEWAY,
                json!({ "error": "upstream_unavailable", "message": error.to_string() }),
            ),
            Error::Json(error) => Self::new(
                StatusCode::BAD_GATEWAY,
                json!({ "error": "upstream_invalid_response", "message": error.to_string() }),
            ),
            Error::Api { title, description } => Self::new(
                StatusCode::BAD_GATEWAY,
                json!({ "error": "upstream_error", "title": title, "message": description }),
            ),
            Error::MissingField(field) => Self::new(
                StatusCode::BAD_GATEWAY,
                json!({ "error": "upstream_error", "message": format!("missing field: {field}") }),
            ),
            Error::MissingTranslation(word) => Self::new(
                StatusCode::BAD_GATEWAY,
                json!({ "error": "upstream_error", "message": format!("missing translation for {word:?}") }),
            ),
            Error::UnknownVariant(value) => Self::new(
                StatusCode::BAD_GATEWAY,
                json!({ "error": "upstream_error", "message": format!("unknown variant: {value}") }),
            ),
            Error::NotImplemented(feature) => Self::new(
                StatusCode::NOT_IMPLEMENTED,
                json!({ "error": "not_implemented", "message": feature }),
            ),
            Error::Url(message) | Error::Header(message) => Self::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "error": "misconfigured", "message": message }),
            ),
        }
    }
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        Self::from_akasha(error)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_uid_is_not_found() {
        let error = ApiError::from_akasha(Error::UidNotFound {
            title: "Not found".to_owned(),
            description: "Provided UID cannot be fetched from Enka Network API".to_owned(),
        });

        assert_eq!(error.status, StatusCode::NOT_FOUND);
        assert_eq!(error.body["error"], "uid_not_found");
    }

    #[test]
    fn malformed_uid_is_a_bad_request() {
        let error = ApiError::from_akasha(Error::IncorrectUidFormat {
            title: "Bad UID".to_owned(),
            description: "The UID format is incorrect".to_owned(),
        });

        assert_eq!(error.status, StatusCode::BAD_REQUEST);
        assert_eq!(error.body["error"], "invalid_uid");
    }

    #[test]
    fn api_failure_is_a_bad_gateway() {
        let error = ApiError::from_akasha(Error::Api {
            title: "Error".to_owned(),
            description: "boom".to_owned(),
        });

        assert_eq!(error.status, StatusCode::BAD_GATEWAY);
        assert_eq!(error.body["error"], "upstream_error");
    }
}
