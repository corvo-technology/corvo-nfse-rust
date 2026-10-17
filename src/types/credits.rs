//! Credit types.

use serde::{Deserialize, Serialize};

/// Current credit balance of the tenant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditBalance {
    /// Owning tenant id.
    pub tenant_id: String,
    /// Remaining credits.
    pub balance: f64,
    /// Billable overage units accrued this period.
    pub overage_units: i64,
    /// Overage amount in cents.
    pub overage_amount_cents: i64,
    /// Hard limit on overage, in credits; `None` for unlimited.
    pub hard_limit: Option<i64>,
}

/// A purchasable (or current) plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditPlan {
    /// Unique id.
    pub id: String,
    /// Stable plan slug.
    pub slug: String,
    /// Display name.
    pub name: String,
    /// Display description.
    pub description: String,
    /// Invoices (credits) included per month.
    pub monthly_credit_allowance: i64,
    /// Invoices per day, if the plan has a daily limit.
    pub daily_limit: Option<i64>,
    /// Maximum active companies.
    pub max_active_companies: i64,
    /// Price of one credit unit in cents.
    pub credit_unit_price_cents: i64,
    /// Sustained throughput in requests per second.
    pub throughput_rps: i64,
    /// Whether the plan includes webhooks.
    pub includes_webhooks: bool,
    /// Whether the plan includes XML download.
    pub includes_xml_download: bool,
    /// Whether the plan includes PDF download.
    pub includes_pdf_download: bool,
    /// Whether the plan includes multiple API keys.
    pub includes_api_keys: bool,
    /// Support tier name.
    pub support_tier: String,
    /// Whether the plan is currently offered.
    pub active: bool,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
    /// Last update timestamp (RFC 3339).
    pub updated_at: String,
}

/// A credit ledger entry. `amount` is in credits (not BRL); fields ending in
/// `_cents` are in centavos.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditTransaction {
    /// Unique id.
    pub id: String,
    /// Owning tenant id.
    pub tenant_id: String,
    /// Transaction type.
    pub r#type: String,
    /// Amount in credits.
    pub amount: f64,
    /// Description, if any.
    #[serde(default)]
    pub description: Option<String>,
    /// What produced this transaction: `subscription`, `avulso` or `bonus`.
    #[serde(default)]
    pub source: Option<String>,
    /// Amount in BRL centavos, if applicable.
    #[serde(default)]
    pub amount_brl_cents: Option<i64>,
    /// Unit price applied, in centavos.
    #[serde(default)]
    pub unit_price_cents_applied: Option<i64>,
    /// Plan id at the time of the transaction.
    #[serde(default)]
    pub plan_id_snapshot: Option<String>,
    /// Invoice this reservation belongs to.
    #[serde(default)]
    pub reserved_invoice_id: Option<String>,
    /// Billing period id.
    #[serde(default)]
    pub period_id: Option<String>,
    /// Whether this is a credit reservation.
    #[serde(default)]
    pub is_reservation: Option<bool>,
    /// Related transaction id, if any.
    #[serde(default)]
    pub related_transaction_id: Option<String>,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
}

/// Filters for listing credit history.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditHistoryParams {
    /// Page size (1–1000, default 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    /// Opaque cursor from a previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Public price catalog entry (`GET /plans`): prices in centavos, monthly cycle only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanQuote {
    /// Plan slug (`free`, `developer`, `growth`, `scale` or `volume`).
    pub slug: String,
    /// Display name.
    pub name: String,
    /// Credits included per month.
    pub monthly_allowance: i64,
    /// Base monthly price in centavos.
    pub base_price_cents: i64,
    /// Price of each credit above the allowance, in centavos.
    pub per_credit_overage_cents: i64,
    /// Monthly price in centavos.
    pub monthly_total_cents: i64,
    /// Currency code (`BRL`).
    pub currency: String,
}

/// Plan recommended for a monthly volume (`GET /plans/quote`). The wire names
/// are snake_case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanRecommendation {
    /// The volume asked about.
    pub volume: i64,
    /// Recommended plan slug.
    pub recommended_plan: String,
    /// Recommended plan name.
    pub plan_name: String,
    /// Credits included per month.
    pub monthly_allowance: i64,
    /// Credits above the allowance.
    pub overage_credits: i64,
    /// Estimated overage cost in centavos.
    pub overage_cost_cents: i64,
    /// Base monthly price in centavos.
    pub base_price_cents: i64,
    /// Estimated total in centavos.
    pub total_estimated_cents: i64,
    /// Currency code (`BRL`).
    pub currency: String,
    /// Billing cycle (`monthly`).
    pub billing_cycle: String,
}
