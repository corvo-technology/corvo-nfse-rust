//! Error type for the Corvo NFS-e SDK.
//!
//! Every fallible operation in this crate returns [`Result<T, CorvoError>`](Result).
//! API, network, and timeout failures are values, never panics.

use serde::{Deserialize, Serialize};

/// A single field-level validation detail attached to an API error.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorDetail {
    /// The request field that failed validation.
    pub field: String,
    /// Stable machine-readable code for this field error.
    pub code: String,
    /// Human-readable description of the field error.
    pub message: String,
}

/// The unified error type of the SDK.
///
/// * [`CorvoError::Api`] — the server answered with a non-2xx status. Carries the
///   HTTP `status`, the server's stable `code` (falling back to `http_<status>`),
///   a `message`, optional field-level `details`, the `request_id`, and — on 429 —
///   `retry_after` in seconds.
/// * [`CorvoError::Network`] — the request never produced an HTTP response
///   (DNS failure, connection refused, TLS error, …).
/// * [`CorvoError::Timeout`] — the request exceeded the configured timeout.
/// * [`CorvoError::InvalidUrl`] — the configured base URL is unusable.
/// * [`CorvoError::Config`] — the client was constructed with invalid configuration
///   (for example a missing API key).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CorvoError {
    /// The server answered with an error response.
    #[error("API error {status} ({code}): {message}")]
    Api {
        /// HTTP status code returned by the server.
        status: u16,
        /// Stable error code (e.g. `ERROR_VALIDATION`), or `http_<status>`.
        code: String,
        /// Human-readable message.
        message: String,
        /// Field-level validation details, when the server provided them.
        details: Vec<ErrorDetail>,
        /// Server request id (body `requestId` or `X-Request-Id` header).
        request_id: Option<String>,
        /// Seconds to wait before retrying, when the server asked for it.
        retry_after: Option<u64>,
    },
    /// The request failed before reaching the server.
    #[error("network error: {0}")]
    Network(String),
    /// The request exceeded the configured timeout.
    #[error("request timed out")]
    Timeout,
    /// The configured base URL could not be parsed or is not allowed.
    #[error("invalid base URL: {0}")]
    InvalidUrl(String),
    /// The client was constructed with invalid configuration.
    #[error("configuration error: {0}")]
    Config(String),
}

impl CorvoError {
    /// The HTTP status code of the failure, or `0` for network/timeout/config errors.
    pub fn status(&self) -> u16 {
        match self {
            CorvoError::Api { status, .. } => *status,
            _ => 0,
        }
    }

    /// The stable error code: the server's `code`, or `network_error` / `timeout`
    /// / `invalid_url` / `configuration` for local failures.
    pub fn code(&self) -> &str {
        match self {
            CorvoError::Api { code, .. } => code.as_str(),
            CorvoError::Network(_) => "network_error",
            CorvoError::Timeout => "timeout",
            CorvoError::InvalidUrl(_) => "invalid_url",
            CorvoError::Config(_) => "configuration",
        }
    }

    /// The server request id, when the error came from the API.
    pub fn request_id(&self) -> Option<&str> {
        match self {
            CorvoError::Api { request_id, .. } => request_id.as_deref(),
            _ => None,
        }
    }

    /// Seconds to wait before retrying (`Retry-After`), when provided.
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            CorvoError::Api { retry_after, .. } => *retry_after,
            _ => None,
        }
    }
}
