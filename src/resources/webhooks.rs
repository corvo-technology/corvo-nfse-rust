//! Webhooks resource (including deliveries): 5 ops + deliveries `iterate`.

use crate::error::CorvoError;
use crate::resources::companies::encode;
use crate::transport::{push_query, Paginator, RequestSpec, Unwrap};
use crate::types::common::Paginated;
use crate::types::webhook::{
    Webhook, WebhookCreateParams, WebhookDelivery, WebhookDeliveryListParams,
    WebhookDeliveryRetryResult, WebhookDeliveryStats, WebhookDeleteResult, WebhookTestResult,
    WebhookWithSecret,
};

/// Filters for listing webhooks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WebhookListParams {
    /// Page size (1–1000, default 50).
    pub limit: Option<u16>,
    /// Opaque cursor from a previous page.
    pub cursor: Option<String>,
}

/// Webhooks resource, obtained via [`Corvo::webhooks()`](crate::Corvo::webhooks).
pub struct Webhooks<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> Webhooks<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// List webhooks (`GET /webhooks`).
    pub async fn list(&self, params: WebhookListParams) -> Result<Paginated<Webhook>, CorvoError> {
        let mut query = Vec::new();
        push_query(&mut query, "limit", params.limit.map(|l| l.to_string()));
        push_query(&mut query, "cursor", params.cursor);
        self.transport
            .list_page("/webhooks", &query, Unwrap::Data)
            .await
    }

    /// Register a webhook (`POST /webhooks`).
    ///
    /// The returned [`WebhookWithSecret`] carries the signing secret — the only
    /// time it is shown. Store it to verify signatures later.
    pub async fn create(
        &self,
        params: &WebhookCreateParams,
    ) -> Result<WebhookWithSecret, CorvoError> {
        let spec = RequestSpec::post("/webhooks").json(params)?;
        self.transport.request_json(&spec).await
    }

    /// Delete a webhook (`DELETE /webhooks/:id`).
    pub async fn remove(&self, id: &str) -> Result<WebhookDeleteResult, CorvoError> {
        let spec = RequestSpec::delete(&format!("/webhooks/{}", encode(id)));
        self.transport.request_json(&spec).await
    }

    /// Send a `webhook.test` event to a webhook (`POST /webhooks/:id/test`).
    pub async fn test(&self, id: &str) -> Result<WebhookTestResult, CorvoError> {
        let spec = RequestSpec::post(&format!("/webhooks/{}/test", encode(id)));
        self.transport.request_json(&spec).await
    }

    /// Deliveries sub-resource.
    pub fn deliveries(&self) -> Deliveries<'c> {
        Deliveries::new(self.transport)
    }
}

/// Webhook deliveries sub-resource, obtained via
/// [`Webhooks::deliveries()`].
pub struct Deliveries<'c> {
    transport: &'c crate::transport::Transport,
}

impl<'c> Deliveries<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// List deliveries (`GET /webhooks/deliveries`).
    pub async fn list(
        &self,
        params: WebhookDeliveryListParams,
    ) -> Result<Paginated<WebhookDelivery>, CorvoError> {
        self.transport
            .list_page("/webhooks/deliveries", &list_query(&params), Unwrap::Data)
            .await
    }

    /// Stream every delivery across pages (`GET /webhooks/deliveries`).
    ///
    /// The first page failure surfaces as the first `Err` from the stream.
    pub fn iterate(&self, params: WebhookDeliveryListParams) -> Paginator<'c, WebhookDelivery> {
        Paginator::new(
            self.transport,
            "/webhooks/deliveries",
            list_query(&params),
            Unwrap::Data,
            params.cursor.clone(),
        )
    }

    /// Delivery stats over the last `hours` (`GET /webhooks/deliveries/stats`).
    pub async fn stats(
        &self,
        hours: Option<u32>,
        environment: Option<crate::types::common::Environment>,
    ) -> Result<WebhookDeliveryStats, CorvoError> {
        let mut query = Vec::new();
        push_query(&mut query, "hours", hours.map(|h| h.to_string()));
        push_query(
            &mut query,
            "environment",
            environment.map(|e| e.as_str().to_owned()),
        );
        let spec = RequestSpec::get("/webhooks/deliveries/stats").query(query);
        self.transport.request_json(&spec).await
    }

    /// Retry a delivery (`POST /webhooks/deliveries/:id/retry`).
    pub async fn retry(&self, delivery_id: &str) -> Result<WebhookDeliveryRetryResult, CorvoError> {
        let spec = RequestSpec::post(&format!(
            "/webhooks/deliveries/{}/retry",
            encode(delivery_id)
        ));
        self.transport.request_json(&spec).await
    }
}

fn list_query(params: &WebhookDeliveryListParams) -> Vec<(String, String)> {
    let mut query = Vec::new();
    push_query(&mut query, "hours", params.hours.map(|h| h.to_string()));
    push_query(
        &mut query,
        "environment",
        params.environment.map(|e| e.as_str().to_owned()),
    );
    push_query(&mut query, "limit", params.limit.map(|l| l.to_string()));
    push_query(&mut query, "cursor", params.cursor.clone());
    query
}
