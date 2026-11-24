//! Payment platform integration types.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::types::common::Environment;
use crate::types::invoice::InvoiceServiceInput;

wire_enum! {
    /// State of an integration connection.
    ConnectionState {
        /// Created, not issuing yet.
        Draft => "draft",
        /// Issuing automatically.
        Active => "active",
        /// Not issuing; events are still recorded.
        Paused => "paused",
        /// The credentials stopped working.
        ReauthRequired => "reauth_required",
        /// Disconnected for good.
        Disconnected => "disconnected",
    }
}

wire_enum! {
    /// State of a received integration event.
    EventState {
        /// Received.
        Received => "received",
        /// Being processed.
        Processing => "processing",
        /// Waiting to be tried again.
        RetryPending => "retry_pending",
        /// Done.
        Processed => "processed",
        /// Ignored.
        Ignored => "ignored",
        /// Needs your attention.
        NeedsAttention => "needs_attention",
        /// Gave up.
        Dead => "dead",
    }
}

wire_enum! {
    /// Commercial state of an operation (sale).
    CommercialState {
        /// Not paid yet.
        Pending => "pending",
        /// Paid.
        Paid => "paid",
        /// Refunded.
        Refunded => "refunded",
        /// Disputed.
        Disputed => "disputed",
        /// Canceled.
        Canceled => "canceled",
    }
}

wire_enum! {
    /// Fiscal state of an operation.
    OperationFiscalStatus {
        /// No invoice yet.
        None => "none",
        /// Blocked, see `block_reason`.
        Blocked => "blocked",
        /// Needs your attention.
        NeedsAttention => "needs_attention",
        /// The invoice was submitted.
        Submitted => "submitted",
    }
}

wire_enum! {
    /// Result of retrying an operation.
    OperationRetryState {
        /// Processed.
        Processed => "processed",
        /// Ignored.
        Ignored => "ignored",
        /// Needs your attention.
        NeedsAttention => "needs_attention",
    }
}

/// A credential field of a provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialField {
    /// Key to send in `credentials`.
    pub key: String,
    /// Label to show.
    pub label: String,
    /// Whether it is a secret.
    pub secret: bool,
    /// Whether Corvo generates it.
    #[serde(default)]
    pub generate: Option<bool>,
    /// Whether it goes in the webhook URL.
    #[serde(default)]
    pub in_webhook_url: Option<bool>,
    /// Whether it is shown only once.
    #[serde(default)]
    pub reveal_once: Option<bool>,
}

/// How to set up the webhook on the platform, by language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebhookInstructions {
    /// Portuguese.
    pub pt: String,
    /// English.
    pub en: String,
}

/// A payment platform Corvo can connect to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationProvider {
    /// Provider id, for example the one to send as `provider`.
    pub id: String,
    /// Name.
    pub name: String,
    /// Version of the connector.
    pub version: String,
    /// Credentials the platform needs.
    pub credential_fields: Vec<CredentialField>,
    /// Flows it supports.
    pub supported_flows: Vec<String>,
    /// How to set up the webhook.
    pub webhook_instructions: WebhookInstructions,
}

/// A connection between a company and a payment platform.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationConnection {
    /// Unique id.
    pub id: String,
    /// Provider id.
    pub provider: String,
    /// Environment.
    pub environment: Environment,
    /// Company that issues.
    pub company_id: String,
    /// State.
    pub state: ConnectionState,
    /// Account on the platform, once identified.
    pub external_account_id: Option<String>,
    /// Sales from this moment on are invoiced.
    pub active_from: Option<String>,
    /// URL to register on the platform.
    pub webhook_url: String,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
}

/// A newly created connection, with the secrets shown only once.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationConnectionCreated {
    /// The connection.
    #[serde(flatten)]
    pub connection: IntegrationConnection,
    /// Secrets generated for it, by key. Store them now.
    pub secrets: BTreeMap<String, String>,
}

/// Parameters for creating a connection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationConnectionCreateParams {
    /// Company that issues.
    pub company_id: String,
    /// Provider id (see `integrations().list_providers()`).
    pub provider: String,
    /// Credentials, by field key.
    pub credentials: BTreeMap<String, String>,
    /// Environment (defaults to the key's).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    /// Only sales from this moment on are invoiced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_from: Option<String>,
}

/// A rule: which service to invoice, per product.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationRule {
    /// Unique id.
    pub id: String,
    /// Whether an invoice without a customer is allowed.
    pub allow_anonymous_customer: bool,
    /// Whether a refund cancels the invoice.
    pub cancel_on_refund: bool,
    /// Product on the platform; `None` is the default rule.
    pub external_product_id: Option<String>,
    /// Whether to e-mail the invoice.
    pub send_email: bool,
    /// The service to invoice (free-form).
    pub service: serde_json::Value,
}

/// A rule to save with [`IntegrationConnections::replace_rules`](crate::IntegrationConnections::replace_rules).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationRuleInput {
    /// Whether to e-mail the invoice.
    pub send_email: bool,
    /// The service to invoice.
    pub service: InvoiceServiceInput,
    /// Whether an invoice without a customer is allowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_anonymous_customer: Option<bool>,
    /// Whether a refund cancels the invoice.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_on_refund: Option<bool>,
    /// Product on the platform; `None` is the default rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_product_id: Option<String>,
}

/// An event received from the platform.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationEvent {
    /// Unique id.
    pub id: String,
    /// Times it was tried.
    pub attempts: i64,
    /// Delivery id on the platform.
    pub delivery_id: String,
    /// Event type on the platform.
    pub external_type: String,
    /// Last error.
    pub last_error: Option<String>,
    /// Operation it produced.
    pub operation_id: Option<String>,
    /// State.
    pub state: EventState,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
}

/// The customer of an operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationOperationCustomer {
    /// Name.
    pub name: Option<String>,
    /// E-mail.
    pub email: Option<String>,
    /// CPF or CNPJ (masked when `tax_id_masked`).
    pub tax_id: Option<String>,
    /// Whether `tax_id` is masked.
    pub tax_id_masked: bool,
}

/// An operation (sale) and its invoice status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationOperation {
    /// Unique id.
    pub id: String,
    /// Amount in centavos.
    pub amount_cents: i64,
    /// Why it is blocked.
    pub block_reason: Option<String>,
    /// Sale state.
    pub commercial_state: CommercialState,
    /// Currency.
    pub currency: String,
    /// Customer, when known.
    pub customer: Option<IntegrationOperationCustomer>,
    /// Invoice state.
    pub fiscal_status: OperationFiscalStatus,
    /// Invoice issued.
    pub invoice_id: Option<String>,
    /// Key on the platform.
    pub operation_key: String,
    /// When it was paid (RFC 3339).
    pub paid_at: Option<String>,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
    /// Last update timestamp (RFC 3339).
    pub updated_at: String,
}

/// Result of retrying an operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationOperationRetry {
    /// What happened.
    pub state: OperationRetryState,
    /// Last error, when it needs attention.
    #[serde(default)]
    pub last_error: Option<String>,
    /// The operation.
    #[serde(default)]
    pub operation_id: Option<String>,
}

/// Customer data to fill in an operation that waits for it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteCustomerParams {
    /// CPF or CNPJ.
    pub tax_id: String,
    /// E-mail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Name (up to 200 characters).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
