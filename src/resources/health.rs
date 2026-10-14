//! Health checks: 2 ops, no API key required.

use serde_json::Value;

use crate::error::CorvoError;
use crate::transport::{RequestSpec, Unwrap};

/// Health resource, obtained via [`Corvo::health()`](crate::Corvo::health).
pub struct Health<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> Health<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Liveness probe (`GET /health/live`). Returns the raw body envelope.
    pub async fn live(&self) -> Result<Value, CorvoError> {
        let spec = RequestSpec::get("/health/live").unwrap_mode(Unwrap::None);
        self.transport.request_json(&spec).await
    }

    /// Readiness probe (`GET /health/ready`). Returns the raw body envelope.
    pub async fn ready(&self) -> Result<Value, CorvoError> {
        let spec = RequestSpec::get("/health/ready").unwrap_mode(Unwrap::None);
        self.transport.request_json(&spec).await
    }
}
