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
    /// Cancelled.
    Cancelled,
}

/// Document generation status of an invoice (`documentsStatus`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentsStatus {
    /// PDF/XML not generated yet.
    Pending,
    /// PDF/XML ready to download.
    Ready,
}

/// E-mail delivery status of an invoice (`emailStatus`, `lastEmailStatus`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmailStatus {
    /// Waiting to be sent.
    Queued,
    /// Sent.
    Sent,
    /// Failed.
    Failed,
}

/// Flag set from the national portal (`portalFlag`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalFlag {
    /// The customer rejected the invoice on the portal.
    TakerRejected,
    /// The portal blocked the invoice.
    Blocked,
}

/// Customer (taker) input for issuing an invoice.
///
/// A Brazilian customer is identified by `tax_id` (CPF or CNPJ); a customer
/// outside Brazil by `foreign` (it cannot be combined with `tax_id` or
/// `address`, and then `name` is required). `name` may be omitted when the
/// customer was already saved: it is filled from the CPF/CNPJ.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCustomerInput {
    /// Customer CPF/CNPJ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// Customer full name or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Customer outside Brazil.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign: Option<ForeignCustomer>,
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
    /// IBGE municipality code (7 digits).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipality_code: Option<String>,
}

/// Why a foreign customer has no tax id (`noTaxIdReason`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoTaxIdReason {
    /// The customer is exempt.
    Exempt,
    /// A tax id is not required.
    NotRequired,
}

/// A customer outside Brazil. Send either `tax_id` (the foreign NIF) or
/// `no_tax_id_reason`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForeignCustomer {
    /// Foreign tax id (up to 40 characters).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// Why there is no tax id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_tax_id_reason: Option<NoTaxIdReason>,
    /// Country code.
    pub country: String,
    /// Postal code (up to 11 characters).
    pub postal_code: String,
    /// City (up to 60 characters).
    pub city: String,
    /// Region or state (up to 60 characters).
    pub region: String,
    /// Street (up to 255 characters).
    pub street: String,
    /// Number (up to 60 characters).
    pub number: String,
    /// District (up to 60 characters).
    pub district: String,
    /// Complement (up to 156 characters).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complement: Option<String>,
}

/// Fiscal classification override of a service (`service.fiscal`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceFiscal {
    /// IBS/CBS class code (6 digits).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class_code: Option<String>,
    /// CST (3 digits).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cst: Option<String>,
    /// NBS code (9 digits).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbs_code: Option<String>,
    /// Operation indicator (6 digits).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_indicator: Option<String>,
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
    /// Fiscal classification override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiscal: Option<ServiceFiscal>,
    /// Saved service tax profile to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_profile_id: Option<String>,
}

/// When an invoice is issued.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvoiceType {
    /// Issue right away (the default).
    #[serde(rename = "IMMEDIATE")]
    Immediate,
    /// Issue once, on the (future) competence date. Creates a schedule, not
    /// an invoice.
    #[serde(rename = "SCHEDULE")]
    Schedule,
    /// Issue repeatedly, see [`RecurrenceOptions`]. Creates a schedule.
    #[serde(rename = "RECURRING")]
    Recurring,
}

/// How often a schedule runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Frequency {
    /// Every week.
    #[serde(rename = "WEEKLY")]
    Weekly,
    /// Every two weeks.
    #[serde(rename = "BIWEEKLY")]
    Biweekly,
    /// Every month.
    #[serde(rename = "MONTHLY")]
    Monthly,
}

/// Recurrence of an invoice with `options.type` `RECURRING`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurrenceOptions {
    /// How often it repeats.
    pub frequency: Frequency,
    /// Last day (`YYYY-MM-DD`) it can run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ends_on: Option<String>,
}

/// Per-invoice option flags.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceOptions {
    /// Email delivery behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<InvoiceEmailOptions>,
    /// `Schedule` or `Recurring` create a schedule instead of an invoice; the
    /// competence date is then the first run and must be from tomorrow on
    /// (up to 15 days ahead, 31 for `Recurring`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub invoice_type: Option<InvoiceType>,
    /// Required for `Recurring`, refused otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<RecurrenceOptions>,
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
    /// Purchase order number (DPS `xPed`, up to 60 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order: Option<String>,
    /// Additional information (DPS `xInfComp`, up to 2000 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
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

/// City and state of a customer snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCustomerSnapshotAddress {
    /// City.
    pub city: String,
    /// State (UF).
    pub state: String,
    /// Country, when not Brazil.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// A customer snapshot as stored on the invoice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCustomerSnapshot {
    /// Customer name.
    pub name: String,
    /// Customer email.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Customer tax id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// Customer address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<InvoiceCustomerSnapshotAddress>,
    /// Customer outside Brazil: `taxId` or `noTaxIdReason` and the address abroad.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foreign: Option<std::collections::BTreeMap<String, String>>,
}

/// A service snapshot as stored on the invoice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceServiceSnapshot {
    /// National service code.
    pub code: String,
    /// Service description.
    pub description: String,
    /// IBGE municipality code.
    pub municipality_code: String,
    /// Fiscal classification used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fiscal: Option<std::collections::BTreeMap<String, String>>,
    /// Notes sent on the invoice (DPS `xInfComp`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Purchase order (DPS `xPed`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purchase_order: Option<String>,
    /// Saved service tax profile used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_profile_id: Option<String>,
}

/// Whether the e-mail to the customer was requested (`emailSnapshot`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvoiceEmailSnapshot {
    /// `true` when the e-mail was requested.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send: Option<bool>,
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
    /// Email settings snapshot.
    #[serde(default)]
    pub email_snapshot: Option<InvoiceEmailSnapshot>,
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
    /// Competence date (ISO 8601).
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
    pub fiscal_provider: String,
    /// Fiscal schema version.
    pub fiscal_schema_version: String,
    /// When the NFS-e was issued (RFC 3339).
    pub issued_at: Option<String>,
    /// When the NFS-e was authorized (RFC 3339).
    pub authorized_at: Option<String>,
    /// Last error code, if issuance failed: a Sefin rule (`E0014`, ...) or a
    /// Corvo code such as `DPS_NUMBER_ALREADY_USED`.
    pub last_error_code: Option<String>,
    /// Last error message, if issuance failed.
    pub last_error_message: Option<String>,
    /// Provider that reported the last error.
    pub last_error_provider: Option<String>,
    /// When documents became ready (RFC 3339).
    pub documents_ready_at: Option<String>,
    /// Last email delivery status.
    pub last_email_status: Option<EmailStatus>,
    /// When the email was last sent (RFC 3339).
    pub last_email_sent_at: Option<String>,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
    /// Last update timestamp (RFC 3339).
    pub updated_at: String,
    /// Document generation status.
    pub documents_status: DocumentsStatus,
    /// Email status.
    pub email_status: Option<EmailStatus>,
    /// Coarse pipeline stage.
    pub stage: InvoiceStage,
    /// When a scheduled invoice is released (RFC 3339); `None` for an immediate one.
    #[serde(default)]
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
    /// On a substitute: the invoice it replaces.
    #[serde(default)]
    pub substitutes_invoice_id: Option<String>,
    /// On a substitute: why it was issued.
    #[serde(default)]
    pub substitution_description: Option<String>,
    /// Flag from the national portal.
    #[serde(default)]
    pub portal_flag: Option<PortalFlag>,
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
    pub from_status: Option<String>,
    /// New status.
    pub to_status: String,
    /// Reason for the transition, if any.
    pub reason: Option<String>,
    /// `portal` when the event came from the national portal.
    #[serde(default)]
    pub source: Option<String>,
    /// Portal event type, for a portal event.
    #[serde(default)]
    pub portal_event: Option<String>,
    /// Portal NSU, for a portal event.
    #[serde(default)]
    pub portal_nsu: Option<String>,
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

/// Result of requesting an invoice e-mail.
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
    /// Only invoices of this customer (CPF or CNPJ, digits).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_tax_id: Option<String>,
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
    /// `VALIDATION`, `CONFIGURATION`, `CLASSIFICATION`, `UNSUPPORTED`,
    /// `CALCULATION`, `INTEGRATION` or `INTERNAL`.
    #[serde(default)]
    pub category: Option<String>,
    /// Extra details (free-form).
    #[serde(default)]
    pub details: Option<Value>,
    /// Whether trying again can work.
    #[serde(default)]
    pub retryable: Option<bool>,
}

/// A withholding of a pre-emission estimate. Amounts are decimal strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithholdingAmount {
    /// `ISS`, `IRRF`, `PIS`, `COFINS`, `CSLL` or `INSS`.
    pub tax: String,
    /// Calculation base.
    pub base: String,
    /// Rate in percent.
    pub rate: String,
    /// Amount withheld.
    pub amount: String,
}

/// A non-blocking fiscal warning of an estimate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FiscalWarning {
    /// Stable code.
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// Pipeline step that warned.
    pub step: String,
}

/// One step of the calculation pipeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculationTrace {
    /// Step name.
    pub step: String,
    /// `applied`, `skipped` or `failed`.
    pub status: String,
    /// Duration in milliseconds.
    pub duration_ms: f64,
    /// Why it was skipped or failed.
    #[serde(default)]
    pub reason: Option<String>,
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
    /// Withholdings.
    #[serde(default)]
    pub withholdings: Vec<WithholdingAmount>,
    /// Net amount to receive, a decimal string.
    #[serde(default)]
    pub net_receivable_amount: Option<String>,
    /// Whether collection is waived (2026 test phase).
    #[serde(default)]
    pub collection_waived: Option<bool>,
    /// Calculation provenance (free-form).
    #[serde(default)]
    pub calculation: Value,
    /// Non-blocking warnings.
    #[serde(default)]
    pub warnings: Vec<FiscalWarning>,
    /// Fields still missing.
    #[serde(default)]
    pub pending_fields: Vec<CalculationPendingField>,
    /// Set when `status` is `"blocked"`.
    #[serde(default)]
    pub blocking: Option<CalculationBlocking>,
    /// Pipeline trace.
    #[serde(default)]
    pub trace: Vec<CalculationTrace>,
    /// Always `true`: the values are not a fiscal document.
    #[serde(default)]
    pub pre_emission: Option<bool>,
}

/// Substitution of an authorized invoice: only the reason, the service
/// description, the purchase order and the notes can change.
///
/// Used by [`Invoices::substitute`](crate::Invoices::substitute).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceSubstituteParams {
    /// Why the invoice is replaced (15 to 255 characters).
    pub description: String,
    /// New notes (up to 2000 characters).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// New purchase order (up to 60 characters).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purchase_order: Option<String>,
    /// New service description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<SubstituteService>,
}

impl InvoiceSubstituteParams {
    /// Substitution with the reason `description` and no other change.
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            notes: None,
            purchase_order: None,
            service: None,
        }
    }
}

/// The service fields that a substitute may change.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubstituteService {
    /// New service description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Window of [`Invoices::overview`](crate::Invoices::overview).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverviewPeriod {
    /// Last 7 days.
    #[serde(rename = "7d")]
    SevenDays,
    /// Last 30 days.
    #[serde(rename = "30d")]
    ThirtyDays,
}

impl OverviewPeriod {
    /// The exact string the API expects on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            OverviewPeriod::SevenDays => "7d",
            OverviewPeriod::ThirtyDays => "30d",
        }
    }
}

/// Counts of an invoice overview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverviewCounts {
    /// Authorized invoices.
    pub authorized: i64,
    /// Invoices being processed.
    pub processing: i64,
    /// Rejected invoices.
    pub rejected: i64,
}

/// Invoices of one day in an overview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverviewDay {
    /// Day (`YYYY-MM-DD`).
    pub date: String,
    /// Invoices of that day.
    pub count: i64,
}

/// Invoice overview of the tenant and environment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceOverview {
    /// Totals by outcome.
    pub counts: OverviewCounts,
    /// Invoices per day.
    pub daily_activity: Vec<OverviewDay>,
    /// Start of the window.
    pub from: String,
    /// `7d` or `30d`.
    pub period: String,
    /// The latest invoices.
    pub recent_invoices: Vec<Invoice>,
    /// Time zone of the days.
    pub timezone: String,
    /// End of the window.
    pub to: String,
}

/// What to do with invoices in `manual_review`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewAction {
    /// Try to issue again.
    Retry,
    /// Discard the invoice.
    Discard,
}

/// Parameters of [`Invoices::review`](crate::Invoices::review).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceReviewParams {
    /// Retry or discard.
    pub action: ReviewAction,
    /// Up to 100 invoice ids in `manual_review`.
    pub invoice_ids: Vec<String>,
}

/// Outcome of one invoice of a review: `accepted` or `skipped`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvoiceReviewOutcome {
    /// Invoice id.
    pub id: String,
    /// `accepted` or `skipped`.
    pub outcome: String,
    /// When skipped: `not_found`, `not_in_manual_review` or `changed`.
    #[serde(default)]
    pub reason: Option<String>,
}

/// Result of [`Invoices::review`](crate::Invoices::review).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvoiceReviewResult {
    /// One entry per invoice sent.
    pub results: Vec<InvoiceReviewOutcome>,
}

/// What `POST /invoices` created: an invoice for `IMMEDIATE`, a schedule for
/// `SCHEDULE` and `RECURRING`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum InvoiceCreated {
    /// The invoice, `queued`.
    Invoice(Box<Invoice>),
    /// The schedule that will issue the invoice.
    Schedule(Box<crate::types::schedule::Schedule>),
}

impl InvoiceCreated {
    /// The invoice, when one was created.
    pub fn invoice(&self) -> Option<&Invoice> {
        match self {
            InvoiceCreated::Invoice(i) => Some(i),
            InvoiceCreated::Schedule(_) => None,
        }
    }

    /// The schedule, when one was created.
    pub fn schedule(&self) -> Option<&crate::types::schedule::Schedule> {
        match self {
            InvoiceCreated::Schedule(s) => Some(s),
            InvoiceCreated::Invoice(_) => None,
        }
    }

    /// The invoice, or `None` when a schedule was created.
    pub fn into_invoice(self) -> Option<Invoice> {
        match self {
            InvoiceCreated::Invoice(i) => Some(*i),
            InvoiceCreated::Schedule(_) => None,
        }
    }

    /// The schedule, or `None` when an invoice was created.
    pub fn into_schedule(self) -> Option<crate::types::schedule::Schedule> {
        match self {
            InvoiceCreated::Schedule(s) => Some(*s),
            InvoiceCreated::Invoice(_) => None,
        }
    }
}

impl<'de> Deserialize<'de> for InvoiceCreated {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        // A schedule has `anchor_on` (snake_case); an invoice never does.
        if value.get("anchor_on").is_some() {
            serde_json::from_value(value)
                .map(|s| InvoiceCreated::Schedule(Box::new(s)))
                .map_err(serde::de::Error::custom)
        } else {
            serde_json::from_value(value)
                .map(|i| InvoiceCreated::Invoice(Box::new(i)))
                .map_err(serde::de::Error::custom)
        }
    }
}

/// Whether the national NFS-e portal is answering (`fiscalPortal_status`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalStatus {
    /// `true` when the portal answers.
    pub available: bool,
    /// Environment the answer is for.
    pub environment: String,
    /// When to try again, while unavailable.
    #[serde(default)]
    pub retry_at: Option<String>,
    /// Since when the portal is unavailable.
    #[serde(default)]
    pub since: Option<String>,
}

/// Count and total of a month package.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageSummary {
    /// Company name.
    pub company_name: String,
    /// Company CNPJ.
    pub company_tax_id: String,
    /// Invoices in the month.
    pub count: i64,
    /// Most invoices a package can hold.
    pub limit: i64,
    /// Month (`YYYY-MM`).
    pub month: String,
    /// Sum of the gross amount of the authorized notes, in BRL, as a decimal
    /// string with two places (for example `"1234.50"`).
    pub total: String,
}

/// A part of the month package (`include`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackagePart {
    /// The XML files.
    Xml,
    /// The PDF files.
    Pdf,
}

impl PackagePart {
    /// The exact string the API expects on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            PackagePart::Xml => "xml",
            PackagePart::Pdf => "pdf",
        }
    }
}
