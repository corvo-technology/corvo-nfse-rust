//! Invoice types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Lifecycle status of an invoice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    /// Accepted, not yet sent to the provider.
    Queued,
    /// Being processed by the fiscal provider.
    Processing,
    /// Authorized by the municipality.
    Authorized,
    /// Rejected by the municipality.
    Rejected,
    /// Waiting to be retried.
    RetryPending,
    /// Needs manual reconciliation.
    ReconciliationRequired,
    /// Held for manual review.
    ManualReview,
    /// Permanently failed.
    Failed,
}

impl InvoiceStatus {
    /// The exact string the API expects on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            InvoiceStatus::Queued => "queued",
            InvoiceStatus::Processing => "processing",
            InvoiceStatus::Authorized => "authorized",
            InvoiceStatus::Rejected => "rejected",
            InvoiceStatus::RetryPending => "retry_pending",
            InvoiceStatus::ReconciliationRequired => "reconciliation_required",
            InvoiceStatus::ManualReview => "manual_review",
            InvoiceStatus::Failed => "failed",
        }
    }
}

/// Coarser pipeline stage of an invoice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStage {
    /// Not yet authorized.
    Pending,
    /// Authorized.
    Authorized,
    /// PDF/XML documents ready.
    DocumentsReady,
    /// Fully delivered.
    Completed,
    /// Terminal failure.
    Failed,
}

/// Customer (taker) input for issuing an invoice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCustomerInput {
    /// Customer full name or company name.
    pub name: String,
    /// Customer CPF/CNPJ, digits only.
    pub tax_id: String,
    /// Customer email, if you want the NFS-e emailed to them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Customer postal address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<InvoiceCustomerAddressInput>,
}

/// Customer address input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCustomerAddressInput {
    /// Street name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// City name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter state code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// CEP (zip code).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    /// Country name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// Service (rps line) description for an invoice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceServiceInput {
    /// Municipal service code (código de tributação nacional).
    pub code: String,
    /// IBGE municipality code where the service is rendered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipality_code: Option<String>,
    /// Human-readable service description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Per-invoice option flags.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceOptions {
    /// Email delivery behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<InvoiceEmailOptions>,
}

/// Email delivery options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceEmailOptions {
    /// Whether to email the NFS-e to the customer on authorization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send: Option<bool>,
}

/// Parameters for creating an invoice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCreateParams {
    /// Company to issue from; omit to use the default company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// Company CNPJ, used instead of `company_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_tax_id: Option<String>,
    /// Your own identifier for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Correlation id for tracing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    /// The customer (taker).
    pub customer: InvoiceCustomerInput,
    /// The service being invoiced.
    pub service: InvoiceServiceInput,
    /// Invoice amount (BRL).
    pub amount: f64,
    /// Competence date (`YYYY-MM-DD`).
    pub competence_date: String,
    /// Email to receive the NFS-e; defaults to the customer email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Extra option flags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<InvoiceOptions>,
}

/// A customer snapshot as stored on the invoice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCustomerSnapshot {
    /// Customer name.
    pub name: String,
    /// Customer email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Customer tax id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// Customer address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<InvoiceCustomerAddressInput>,
}

/// A service snapshot as stored on the invoice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceServiceSnapshot {
    /// Municipal service code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Service description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// IBGE municipality code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipality_code: Option<String>,
}

/// An issued (or in-flight) invoice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    /// Unique id.
    pub id: String,
    /// Issuing company id.
    pub company_id: String,
    /// Owning tenant id.
    pub tenant_id: String,
    /// Issuance environment (`test` or `live`).
    pub environment: String,
    /// Document key version.
    pub document_key_version: i64,
    /// Email settings snapshot.
    pub email_snapshot: Value,
    /// Your external identifier, if set.
    pub external_id: Option<String>,
    /// Idempotency key used at creation, if any.
    pub idempotency_key: Option<String>,
    /// Lifecycle status.
    pub status: InvoiceStatus,
    /// Customer snapshot.
    pub customer_snapshot: InvoiceCustomerSnapshot,
    /// Service snapshot.
    pub service_snapshot: InvoiceServiceSnapshot,
    /// Tax computation snapshot.
    pub tax_snapshot: Value,
    /// Invoice amount (BRL).
    pub amount: f64,
    /// Currency code.
    pub currency: String,
    /// Competence date (`YYYY-MM-DD`).
    pub competence_date: String,
    /// DPS id, once issued.
    pub dps_id: Option<String>,
    /// DPS series, once issued.
    pub dps_series: Option<String>,
    /// DPS number, once issued.
    pub dps_number: Option<i64>,
    /// NFS-e access key (chave de acesso), once authorized.
    pub access_key: Option<String>,
    /// NFS-e number, once authorized.
    pub nfse_number: Option<i64>,
    /// Fiscal provider used.
    pub fiscal_provider: Option<String>,
    /// Fiscal schema version.
    pub fiscal_schema_version: Option<String>,
    /// When the NFS-e was issued (RFC 3339).
    pub issued_at: Option<String>,
    /// When the NFS-e was authorized (RFC 3339).
    pub authorized_at: Option<String>,
    /// Last error code, if issuance failed.
    pub last_error_code: Option<String>,
    /// Last error message, if issuance failed.
    pub last_error_message: Option<String>,
    /// Provider that reported the last error.
    pub last_error_provider: Option<String>,
    /// When documents became ready (RFC 3339).
    pub documents_ready_at: Option<String>,
    /// Last email delivery status.
    pub last_email_status: Option<String>,
    /// When the email was last sent (RFC 3339).
    pub last_email_sent_at: Option<String>,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
    /// Last update timestamp (RFC 3339).
    pub updated_at: String,
    /// Document generation status.
    pub documents_status: String,
    /// Email status.
    pub email_status: Option<String>,
    /// Coarse pipeline stage.
    pub stage: InvoiceStage,
}

/// A status-transition event on an invoice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceEvent {
    /// Unique id.
    pub id: String,
    /// The invoice this event belongs to.
    pub invoice_id: String,
    /// Previous status, if any.
    pub from_status: Option<InvoiceStatus>,
    /// New status.
    pub to_status: InvoiceStatus,
    /// Reason for the transition, if any.
    pub reason: Option<String>,
    /// When the transition happened (RFC 3339).
    pub created_at: String,
}

/// A pre-signed download URL for an invoice document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceDownloadUrl {
    /// The URL to download.
    pub download_url: String,
    /// When the URL expires (RFC 3339).
    pub expires_at: String,
    /// Suggested file name.
    pub file_name: String,
    /// MIME type of the document.
    pub mime_type: String,
}

/// Result of sending an invoice by email.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceEmailResult {
    /// Email job id.
    pub email_id: String,
    /// Email job status.
    pub status: String,
}

/// Filters for listing invoices.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceListParams {
    /// Only invoices of this company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// Only invoices in this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InvoiceStatus>,
    /// Only invoices created at or after this RFC 3339 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Only invoices created at or before this RFC 3339 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Only invoices with this external id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Page size (1–1000, default 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    /// Opaque cursor from a previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Document format for download URLs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadFormat {
    /// PDF rendering of the NFS-e.
    Pdf,
    /// XML (DPS/NFS-e) document.
    Xml,
}

impl DownloadFormat {
    /// The exact string the API expects on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            DownloadFormat::Pdf => "pdf",
            DownloadFormat::Xml => "xml",
        }
    }
}
