//! Billing types.

use serde::{Deserialize, Serialize};

/// Plan details returned by the billing plan endpoint.
///
/// Note: this endpoint's wire format uses `snake_case` field names.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BillingPlanInfo {
    /// Plan slug.
    pub slug: String,
    /// Plan display name.
    pub name: String,
    /// Invoices included per month.
    pub monthly_allowance: i64,
    /// Invoices per day, if the plan is metered daily.
    pub daily_limit: Option<i64>,
    /// Maximum active companies.
    pub max_active_companies: i64,
    /// Monthly price in cents.
    pub monthly_price_cents: i64,
}

/// Subscription state returned by the billing plan endpoint.
///
/// Note: this endpoint's wire format uses `snake_case` field names.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BillingSubscription {
    /// Subscription status.
    pub status: String,
    /// When the current period is paid until (RFC 3339).
    pub paid_until: Option<String>,
    /// Whether the subscription cancels at period end.
    pub cancel_at_period_end: bool,
    /// Slug of the scheduled plan change, if any.
    pub scheduled_plan: Option<String>,
    /// Whether the tenant is blocked from issuing.
    pub blocked: bool,
}

/// Usage counters returned by the billing plan endpoint.
///
/// Note: this endpoint's wire format uses `snake_case` field names.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BillingUsage {
    /// Billable overage units.
    pub overage_units: i64,
    /// Overage amount in cents.
    pub overage_amount_cents: i64,
    /// Hard limit on overage; `None` for unlimited.
    pub hard_limit: Option<i64>,
    /// Credits consumed this period.
    pub credits_used: i64,
    /// Credits reserved for in-flight work.
    pub credits_reserved: i64,
    /// Credits remaining.
    pub credits_remaining: i64,
    /// Monthly allowance.
    pub monthly_allowance: i64,
    /// Usage so far today.
    pub daily_usage: i64,
    /// Daily limit, if metered daily.
    pub daily_limit: Option<i64>,
    /// Active companies.
    pub active_companies: i64,
    /// Maximum active companies.
    pub max_active_companies: i64,
    /// Current billing period start (RFC 3339).
    pub period_start: Option<String>,
    /// Current billing period end (RFC 3339).
    pub period_end: Option<String>,
}

/// Full billing plan response: plan, subscription, and usage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingPlanResponse {
    /// The tenant's plan.
    pub plan: BillingPlanInfo,
    /// The tenant's subscription state.
    pub subscription: BillingSubscription,
    /// Current usage counters.
    pub usage: BillingUsage,
}

/// Quota status from the billing usage endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotaStatus {
    /// Whether the tenant can currently emit.
    pub can_emit: bool,
    /// Why it cannot: `no_credits`, `daily_limit_exceeded`,
    /// `company_limit_exceeded` or `plan_blocked`.
    #[serde(default)]
    pub reason: Option<String>,
    /// Credits remaining.
    pub credits_remaining: i64,
    /// Credits consumed this period.
    pub credits_used: i64,
    /// Credits reserved for in-flight work.
    pub credits_reserved: i64,
    /// Monthly allowance.
    pub monthly_allowance: i64,
    /// Usage so far today.
    pub daily_usage: i64,
    /// Daily limit, if metered daily.
    pub daily_limit: Option<i64>,
    /// Active companies.
    pub active_companies: i64,
    /// Maximum active companies.
    pub max_active_companies: i64,
    /// Current billing period start (RFC 3339).
    pub period_start: Option<String>,
    /// Current billing period end (RFC 3339).
    pub period_end: Option<String>,
}
