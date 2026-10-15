//! # corvo-nfse
//!
//! Idiomatic Rust SDK for the Corvo NFS-e API — issue, list, download, and
//! reconcile Brazilian electronic service invoices (NFS-e).
//!
//! * **Self-hosted first** — point it at your instance with a base URL; the
//!   default `https://api.corvo.tech` only applies when you don't.
//! * **Result-based errors** — every call returns `Result<T, CorvoError>`.
//!   API, network, and timeout failures are values with the server's stable
//!   `code`, HTTP `status`, `request_id`, and field-level `details`.
//! * **Pagination you don't have to think about** — every list returns
//!   `{ data, pagination }`; `iterate()` streams every page for you.
//!
//! ## Quickstart
//!
//! ```no_run
//! # async fn run() -> Result<(), corvo_nfse::CorvoError> {
//! use corvo_nfse::Corvo;
//!
//! let corvo = Corvo::builder()
//!     .api_key("corvo_live_xxx") // or set CORVO_API_KEY
//!     .build()?;
//!
//! let page = corvo.companies().list(Default::default()).await?;
//! println!("{} companies", page.data.len());
//! # Ok(())
//! # }
//! ```
//!
//! ## Webhook signatures
//!
//! ```no_run
//! use corvo_nfse::verify_signature;
//!
//! # fn handle(raw_body: &[u8], header: Option<&str>, secret: &str) -> bool {
//! verify_signature(raw_body, header, secret)
//! # }
//! ```

#![forbid(unsafe_code)]

pub mod client;
pub mod error;
pub mod resources;
pub mod transport;
pub mod types;
pub mod webhooks;

pub use client::{Corvo, CorvoBuilder, DEFAULT_BASE_URL};
pub use error::{CorvoError, ErrorDetail};
pub use resources::companies::CompanyListParams;
pub use resources::credits::CreditPlanListParams;
pub use resources::webhooks::WebhookListParams;
pub use resources::{
    Billing, Certificates, Companies, Credits, Deliveries, Health, Invoices, Webhooks,
};
pub use transport::{DownloadedFile, Paginator};
pub use types::{
    BillingPlanInfo, BillingPlanResponse, BillingSubscription, BillingUsage, Certificate,
    CertificateUploadParams, Company, CompanyAddressInput, CompanyCreateParams,
    CompanyUpdateParams, CreditBalance, CreditHistoryParams, CreditPlan, CreditTransaction,
    DownloadFormat, Environment, Invoice, InvoiceCreateParams, InvoiceCustomerAddressInput,
    InvoiceCustomerInput, InvoiceCustomerSnapshot, InvoiceDownloadUrl, InvoiceEmailOptions,
    InvoiceEmailResult, InvoiceEvent, InvoiceListParams, InvoiceOptions, InvoiceServiceInput,
    InvoiceServiceSnapshot, InvoiceStage, InvoiceStatus, Paginated, Pagination, QuotaStatus,
    TaxRegime, Webhook, WebhookCreateParams, WebhookDeleteResult, WebhookDelivery,
    WebhookDeliveryListParams, WebhookDeliveryRetryResult, WebhookDeliveryStats, WebhookTestResult,
    WebhookWithSecret, WEBHOOK_EVENTS,
};
pub use webhooks::verify_signature;

/// The SDK version (kept in sync with `Cargo.toml`).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
