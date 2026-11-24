//! Plans resource: the public price catalog (no API key needed).

use crate::error::CorvoError;
use crate::resources::companies::encode;
use crate::transport::{push_query, RequestSpec, Unwrap};
use crate::types::common::Paginated;
use crate::types::credits::{PlanPrice, PlanRecommendation};

/// Plans resource, obtained via [`Corvo::plans()`](crate::Corvo::plans).
pub struct Plans<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> Plans<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// List plan prices, from the smallest to the largest allowance (`GET /plans`).
    ///
    /// `cycle` is `monthly` (default) or `annual` (discounted).
    pub async fn list(
        &self,
        limit: Option<u16>,
        cursor: Option<&str>,
        cycle: Option<&str>,
    ) -> Result<Paginated<PlanPrice>, CorvoError> {
        let mut query = Vec::new();
        push_query(&mut query, "limit", limit.map(|l| l.to_string()));
        push_query(&mut query, "cursor", cursor.map(str::to_owned));
        push_query(&mut query, "cycle", cycle.map(str::to_owned));
        self.transport
            .list_page("/plans", &query, Unwrap::None)
            .await
    }

    /// The price of one plan by slug (`GET /plans/:slug`).
    pub async fn get(&self, slug: &str, cycle: Option<&str>) -> Result<PlanPrice, CorvoError> {
        let mut query = Vec::new();
        push_query(&mut query, "cycle", cycle.map(str::to_owned));
        let spec = RequestSpec::get(&format!("/plans/{}", encode(slug))).query(query);
        self.transport.request_json(&spec).await
    }

    /// The plan recommended for `volume` invoices per month (`GET /plans/quote`).
    ///
    /// Returns `None` (no error) when the volume is missing or not positive.
    pub async fn quote(&self, volume: u32) -> Result<Option<PlanRecommendation>, CorvoError> {
        let mut query = Vec::new();
        push_query(&mut query, "volume", Some(volume.to_string()));
        let spec = RequestSpec::get("/plans/quote").query(query);
        self.transport.request_json(&spec).await
    }
}
