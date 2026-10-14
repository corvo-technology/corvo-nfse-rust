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
    /// Invoices per day included.
    pub daily_limit: i64,
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

/// A credit ledger entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditTransaction {
    /// Unique id.
    pub id: String,
    /// Transaction type.
    pub r#type: String,
    /// Signed amount of the transaction.
    pub amount: f64,
    /// Change to the credit balance.
    pub credit_delta: f64,
    /// Balance immediately after this transaction.
    pub balance_after: f64,
    /// Environment (`test` or `live`).
    pub environment: String,
    /// What produced this transaction.
    pub source: String,
    /// Related entity type, if any.
    pub reference_type: Option<String>,
    /// Related entity id, if any.
    pub reference_id: Option<String>,
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
