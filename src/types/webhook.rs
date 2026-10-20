//! Webhook types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::common::Environment;

/// Every event a webhook can subscribe to. `webhook.test` is only sent by
/// `webhooks().test()` and cannot be subscribed.
pub const WEBHOOK_EVENTS: [&str; 11] = [
    "nfse.invoice.authorized",
    "nfse.invoice.rejected",
    "nfse.invoice.failed",
    "nfse.invoice.reconciliation_required",
    "nfse.invoice.manual_review",
    "nfse.invoice.documents_ready",
    "nfse.invoice.email_sent",
    "nfse.invoice.email_failed",
    "nfse.invoice.fiscal_discrepancy",
    "nfse.invoice.cancelled",
    "nfse.invoice.cancellation_rejected",
];

/// A registered webhook endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    /// Unique id.
    pub id: String,
    /// Target URL.
    pub url: String,
    /// Subscribed event types.
    pub events: Vec<String>,
    /// Whether the endpoint is active.
    pub active: bool,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
    /// Last update timestamp (RFC 3339).
    pub updated_at: String,
}

/// A newly created webhook, including its signing secret.
///
/// The `secret` is returned only once, at creation time. Store it to verify
/// signatures with [`verify_signature`](crate::verify_signature).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookWithSecret {
    /// Unique id.
    pub id: String,
    /// Target URL.
    pub url: String,
    /// Subscribed event types.
    pub events: Vec<String>,
    /// Whether the endpoint is active.
    pub active: bool,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
    /// Last update timestamp (RFC 3339).
    pub updated_at: String,
    /// The signing secret, shown only at creation.
    pub secret: String,
}

/// Parameters for creating a webhook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookCreateParams {
    /// Target URL that will receive the events.
    pub url: String,
    /// Event types to subscribe to; see [`WEBHOOK_EVENTS`].
    pub events: Vec<String>,
}

/// A delivery attempt (or record) of a webhook event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookDelivery {
    /// Unique id.
    pub id: String,
    /// Owning tenant id.
    pub tenant_id: String,
    /// Webhook configuration id.
    pub webhook_config_id: String,
    /// Event type delivered.
    pub event_type: String,
    /// Environment (`test` or `live`).
    pub environment: String,
    /// Target URL.
    pub url: String,
    /// The JSON payload that was sent.
    pub payload: Value,
    /// HTTP status code returned by the target, if any.
    pub status_code: Option<i64>,
    /// Response body returned by the target, if any.
    pub response_body: Option<Value>,
    /// Error message when delivery failed.
    pub error_message: Option<String>,
    /// Delivery status.
    pub status: String,
    /// Delivery duration in milliseconds, if measured.
    pub duration_ms: Option<i64>,
    /// Number of retries attempted so far.
    pub retry_count: i64,
    /// When the next retry is scheduled (RFC 3339).
    pub next_retry_at: Option<String>,
    /// Last error recorded, if any.
    pub last_error: Option<String>,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
}

/// Aggregated delivery counters over a recent window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookDeliveryStats {
    /// Total deliveries in the window.
    pub total: i64,
    /// Deliveries that succeeded.
    pub sent: i64,
    /// Deliveries that failed.
    pub failed: i64,
    /// Deliveries still pending.
    pub pending: i64,
    /// Deliveries delayed for retry.
    pub delayed: i64,
}

/// Filters for listing webhook deliveries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookDeliveryListParams {
    /// Only deliveries from the last N hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<u32>,
    /// Only deliveries in this environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    /// Page size (1–1000, default 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    /// Opaque cursor from a previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Result of deleting a webhook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookDeleteResult {
    /// Whether the webhook was deleted.
    pub deleted: bool,
}

/// Result of sending a test event to a webhook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookTestResult {
    /// Whether the test delivery was dispatched.
    pub sent: bool,
}

/// Result of retrying a webhook delivery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookDeliveryRetryResult {
    /// Whether a retry was scheduled.
    pub retried: bool,
}
