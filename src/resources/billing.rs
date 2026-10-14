//! Billing resource: 2 ops.

use crate::error::CorvoError;
use crate::transport::{RequestSpec, Unwrap};
use crate::types::billing::{BillingPlanResponse, QuotaStatus};

/// Billing resource, obtained via [`Corvo::billing()`](crate::Corvo::billing).
pub struct Billing<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> Billing<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Plan, subscription, and usage (`GET /billing/plan`).
    pub async fn plan(&self) -> Result<BillingPlanResponse, CorvoError> {
        let spec = RequestSpec::get("/billing/plan");
        self.transport.request_json(&spec).await
    }

    /// Quota status (`GET /billing/usage`).
    pub async fn usage(&self) -> Result<QuotaStatus, CorvoError> {
        let spec = RequestSpec::get("/billing/usage");
        self.transport.request_json(&spec).await
    }
}
