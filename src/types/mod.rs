//! Serde types mirroring the Corvo NFS-e wire format.
//!
//! Field names follow the API's `camelCase` convention via
//! `#[serde(rename_all = "camelCase")]` — except the billing plan endpoint,
//! whose nested objects use `snake_case` and are annotated as such.

pub mod billing;
pub mod certificate;
pub mod common;
pub mod company;
pub mod credits;
pub mod invoice;
pub mod webhook;

pub use billing::{
    BillingPlanInfo, BillingPlanResponse, BillingSubscription, BillingUsage, QuotaStatus,
};
pub use certificate::{Certificate, CertificateUploadParams};
pub use common::{Environment, Paginated, Pagination};
pub use company::{
    Company, CompanyAddressInput, CompanyCreateParams, CompanyUpdateParams, TaxRegime,
};
pub use credits::{
    CreditBalance, CreditHistoryParams, CreditPlan, CreditTransaction, PlanQuote,
    PlanRecommendation,
};
pub use invoice::{
    CalculatedTaxAmount, CalculatedTaxes, CalculationBlocking, CalculationPendingField,
    DownloadFormat, Invoice, InvoiceCalculation, InvoiceCreateParams, InvoiceCustomerAddressInput,
    InvoiceCustomerInput, InvoiceCustomerSnapshot, InvoiceDownloadUrl, InvoiceEmailOptions,
    InvoiceEmailResult, InvoiceEvent, InvoiceListParams, InvoiceOptions, InvoiceServiceInput,
    InvoiceServiceSnapshot, InvoiceStage, InvoiceStatus, InvoiceType,
};
pub use webhook::{
    Webhook, WebhookCreateParams, WebhookDeleteResult, WebhookDelivery, WebhookDeliveryListParams,
    WebhookDeliveryRetryResult, WebhookDeliveryStats, WebhookTestResult, WebhookWithSecret,
    WEBHOOK_EVENTS,
};
