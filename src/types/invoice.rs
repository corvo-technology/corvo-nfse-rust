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
    /// Cancellation requested; the NFS-e cancellation event is being processed
    /// by the Sefin.
    Cancelling,
    /// Cancelled (terminal).
    Cancelled,
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
            InvoiceStatus::Cancelling => "cancelling",
            InvoiceStatus::Cancelled => "cancelled",
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
///
/// When a customer is sent, `tax_id` (CPF or CNPJ) is required; leave
/// [`InvoiceCreateParams::customer`] as `None` for an invoice without one.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCustomerInput {
    /// Customer CPF/CNPJ.
    pub tax_id: String,
    /// Customer full name or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Customer email; required when the invoice must be emailed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Customer postal address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<InvoiceCustomerAddressInput>,
}

/// Customer address input.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
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

/// Service description for an invoice. Every field is optional.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceServiceInput {
    /// National service code (cTribNac, 6 digits). Defaults to the company's
    /// `default_service_code`, then to `100901`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// IBGE municipality code (7 digits) where the service is rendered.
    /// Defaults to the company's city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipality_code: Option<String>,
    /// Human-readable service description; defaults to the name of the code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// When an invoice is issued.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvoiceType {
    /// Issue right away (the default).
    #[serde(rename = "IMMEDIATE")]
    Immediate,
    /// Issue on the (future) competence date.
    #[serde(rename = "SCHEDULE")]
    Schedule,
}

/// Per-invoice option flags.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceOptions {
    /// Email delivery behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<InvoiceEmailOptions>,
    /// `Schedule` with a future competence date schedules the issuance (up to
    /// 15 days ahead by default).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub invoice_type: Option<InvoiceType>,
}

/// Email delivery options.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceEmailOptions {
    /// Whether to email the NFS-e to the customer on authorization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send: Option<bool>,
}

/// Parameters for creating (or calculating) an invoice.
///
/// Identify the company with `company_id` or `company_tax_id`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCreateParams {
    /// Company to issue from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// Company CNPJ, used instead of `company_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_tax_id: Option<String>,
    /// Your own identifier for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The customer (taker). Optional: omit it for an invoice without one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<InvoiceCustomerInput>,
    /// The service being invoiced.
    pub service: InvoiceServiceInput,
    /// Invoice amount in BRL: positive, up to 2 decimal places.
    pub amount: f64,
    /// Competence date (`YYYY-MM-DD`).
    pub competence_date: String,
    /// Unconditional discount in BRL, from 0 up to `amount`, at most 2 decimal
    /// places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
    /// Deduction from the ISS base declared by the issuer, in BRL: greater than
    /// 0 and less than `amount` minus `discount`, at most 2 decimal places (`0`
    /// is the same as omitting it). The invoice is still issued for the full
    /// amount; the issuer is responsible for the declared value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduction: Option<f64>,
    /// Request the email notification (default `false`); `options.email.send`
    /// wins when set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<bool>,
    /// Extra option flags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<InvoiceOptions>,
}

/// A customer snapshot as stored on the invoice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCustomerSnapshot {
    /// Customer name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    /// Unconditional discount (BRL), when one was declared.
    #[serde(default)]
    pub discount: Option<f64>,
    /// Deduction from the ISS base (BRL), when one was declared.
    #[serde(default)]
    pub deduction: Option<f64>,
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
    /// When a scheduled invoice is released (RFC 3339); `None` for an immediate one.
    pub scheduled_for: Option<String>,
    /// Why the invoice was cancelled, once a cancellation was requested.
    #[serde(default)]
    pub cancellation_reason: Option<InvoiceCancellationReason>,
    /// Free-text cancellation description, if one was sent.
    #[serde(default)]
    pub cancellation_description: Option<String>,
    /// When the invoice was cancelled (RFC 3339).
    #[serde(default)]
    pub cancelled_at: Option<String>,
    /// When the cancellation was requested (RFC 3339).
    #[serde(default)]
    pub cancellation_requested_at: Option<String>,
}

/// Reason for cancelling an invoice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceCancellationReason {
    /// The invoice was issued with an error (wrong amount, customer, ...).
    IssueError,
    /// The service was not provided.
    ServiceNotProvided,
    /// Any other reason; explain it in the description.
    Other,
}

impl InvoiceCancellationReason {
    /// The exact string the API expects on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            InvoiceCancellationReason::IssueError => "issue_error",
            InvoiceCancellationReason::ServiceNotProvided => "service_not_provided",
            InvoiceCancellationReason::Other => "other",
        }
    }
}

/// Parameters for cancelling an invoice with
/// [`Invoices::cancel`](crate::Invoices::cancel).
///
/// `description` (up to 255 characters) is required by the API, with 15 to 255
/// characters, to cancel an `authorized` invoice or one still `processing`; it
/// is optional for a `queued` invoice. The SDK does not validate it locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCancelParams {
    /// Why the invoice is being cancelled.
    pub reason: InvoiceCancellationReason,
    /// Free-text explanation (15 to 255 characters when required).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl InvoiceCancelParams {
    /// Cancellation with `reason` and no description.
    pub fn new(reason: InvoiceCancellationReason) -> Self {
        Self {
            reason,
            description: None,
        }
    }

    /// Set the description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// One item of [`Invoices::create_batch`](crate::Invoices::create_batch): the
/// fields of [`InvoiceCreateParams`] plus its own idempotency key.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceBatchItem {
    /// The invoice to issue.
    #[serde(flatten)]
    pub invoice: InvoiceCreateParams,
    /// Up to 255 characters. Repeating it with the same content returns the
    /// saved invoice with `replayed: true` instead of issuing again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

/// Why a batch item was rejected. Rejected items are not saved.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoiceBatchError {
    /// Stable error code, e.g. `ERROR_VALIDATION`.
    pub code: String,
    /// English message meant for diagnostics.
    pub message: String,
    /// Field-level details, when the error has them.
    #[serde(default)]
    pub details: Option<Value>,
}

/// Outcome of one batch item; `index` is its position in the request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceBatchResult {
    /// Position of the item in the request, starting at 0.
    pub index: u32,
    /// `accepted` or `rejected`.
    pub status: String,
    /// `true` when the item repeated an idempotency key already saved with the
    /// same content.
    #[serde(default)]
    pub replayed: Option<bool>,
    /// The saved invoice, when `status` is `accepted`.
    #[serde(default)]
    pub invoice: Option<Invoice>,
    /// The reason, when `status` is `rejected`.
    #[serde(default)]
    pub error: Option<InvoiceBatchError>,
}

/// Result of [`Invoices::create_batch`](crate::Invoices::create_batch).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceBatchSummary {
    /// Items sent.
    pub total: u32,
    /// Items saved.
    pub accepted: u32,
    /// Items rejected (not saved).
    pub rejected: u32,
    /// One entry per item, in the order sent.
    pub results: Vec<InvoiceBatchResult>,
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

/// One tax of a pre-emission estimate. Amounts are decimal strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculatedTaxAmount {
    /// Calculation base.
    pub base: String,
    /// Rate in percent; absent on `ibs_total`, which sums state and municipal IBS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    /// Tax amount.
    pub amount: String,
}

/// The taxes of a pre-emission estimate.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculatedTaxes {
    /// ISS (municipal service tax).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iss: Option<CalculatedTaxAmount>,
    /// CBS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cbs: Option<CalculatedTaxAmount>,
    /// State IBS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ibs_state: Option<CalculatedTaxAmount>,
    /// Municipal IBS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ibs_city: Option<CalculatedTaxAmount>,
    /// Total IBS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ibs_total: Option<CalculatedTaxAmount>,
}

/// Why an estimate is blocked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalculationBlocking {
    /// Stable code, for example `TAX_RATE_NOT_FOUND`.
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// Pipeline step that blocked.
    pub step: String,
}

/// A field missing for the estimate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalculationPendingField {
    /// The missing field.
    pub field: String,
    /// Why it is needed.
    pub reason: String,
}

/// Pre-emission estimate returned by `invoices().calculate()`.
///
/// `status` is `"ready"` or `"blocked"`: a fiscal gap is data, not an error.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCalculation {
    /// `"ready"` or `"blocked"`.
    pub status: String,
    /// Legal notice: the values are not a fiscal document.
    #[serde(default)]
    pub disclaimer: Option<String>,
    /// Competence date (`YYYY-MM-DD`).
    #[serde(default)]
    pub competence_date: Option<String>,
    /// Tax regime used.
    #[serde(default)]
    pub tax_regime: Option<String>,
    /// Service classification (free-form).
    #[serde(default)]
    pub classification: Value,
    /// Which taxes apply and why (free-form).
    #[serde(default)]
    pub applicability: Value,
    /// The calculated taxes.
    #[serde(default)]
    pub taxes: CalculatedTaxes,
    /// Withholdings (free-form).
    #[serde(default)]
    pub withholdings: Vec<Value>,
    /// Net amount to receive, a decimal string.
    #[serde(default)]
    pub net_receivable_amount: Option<String>,
    /// Whether collection is waived (2026 test phase).
    #[serde(default)]
    pub collection_waived: Option<bool>,
    /// Calculation provenance (free-form).
    #[serde(default)]
    pub calculation: Value,
    /// Non-blocking warnings (free-form).
    #[serde(default)]
    pub warnings: Vec<Value>,
    /// Fields still missing.
    #[serde(default)]
    pub pending_fields: Vec<CalculationPendingField>,
    /// Set when `status` is `"blocked"`.
    #[serde(default)]
    pub blocking: Option<CalculationBlocking>,
    /// Pipeline trace (free-form).
    #[serde(default)]
    pub trace: Vec<Value>,
}
