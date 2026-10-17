//! Credits resource: 4 ops.

use crate::error::CorvoError;
use crate::transport::{push_query, Paginator, RequestSpec, Unwrap};
use crate::types::common::Paginated;
use crate::types::credits::{CreditBalance, CreditHistoryParams, CreditPlan, CreditTransaction};

/// Filters for listing credit plans.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CreditPlanListParams {
    /// Page size (1–1000, default 50).
    pub limit: Option<u16>,
    /// Opaque cursor from a previous page.
    pub cursor: Option<String>,
}

/// Credits resource, obtained via [`Corvo::credits()`](crate::Corvo::credits).
pub struct Credits<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> Credits<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Current credit balance (`GET /credits`).
    pub async fn balance(&self) -> Result<CreditBalance, CorvoError> {
        let spec = RequestSpec::get("/credits");
        self.transport.request_json(&spec).await
    }

    /// List purchasable plans (`GET /credits/plans`).
    pub async fn plans(
        &self,
        params: CreditPlanListParams,
    ) -> Result<Paginated<CreditPlan>, CorvoError> {
        let mut query = Vec::new();
        push_query(&mut query, "limit", params.limit.map(|l| l.to_string()));
        push_query(&mut query, "cursor", params.cursor);
        self.transport
            .list_page("/credits/plans", &query, Unwrap::None)
            .await
    }

    /// Credit ledger history (`GET /credits/history`).
    pub async fn history(
        &self,
        params: CreditHistoryParams,
    ) -> Result<Paginated<CreditTransaction>, CorvoError> {
        let mut query = Vec::new();
        push_query(&mut query, "limit", params.limit.map(|l| l.to_string()));
        push_query(&mut query, "cursor", params.cursor);
        self.transport
            .list_page("/credits/history", &query, Unwrap::None)
            .await
    }

    /// Stream the credit ledger across pages (`GET /credits/history`).
    ///
    /// The first page failure surfaces as the first `Err` from the stream.
    pub fn iterate_history(&self, params: CreditHistoryParams) -> Paginator<'c, CreditTransaction> {
        let mut query = Vec::new();
        push_query(&mut query, "limit", params.limit.map(|l| l.to_string()));
        Paginator::new(
            self.transport,
            "/credits/history",
            query,
            Unwrap::None,
            params.cursor,
        )
    }
}
