//! Payment platform integrations: 1 + 12 + 2 ops.

use crate::error::CorvoError;
use crate::resources::companies::encode;
use crate::transport::{push_query, RequestSpec};
use crate::types::integration::{
    CompleteCustomerParams, IntegrationConnection, IntegrationConnectionCreateParams,
    IntegrationConnectionCreated, IntegrationEvent, IntegrationOperation,
    IntegrationOperationRetry, IntegrationProvider, IntegrationRule, IntegrationRuleInput,
};

use std::collections::BTreeMap;

use serde_json::json;

/// Providers of payment platforms, obtained via
/// [`Corvo::integrations()`](crate::Corvo::integrations).
pub struct Integrations<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> Integrations<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// The payment platforms Corvo can connect to (`GET /integrations/providers`).
    pub async fn list_providers(&self) -> Result<Vec<IntegrationProvider>, CorvoError> {
        let spec = RequestSpec::get("/integrations/providers");
        self.transport.request_json(&spec).await
    }
}

/// Integration connections, obtained via
/// [`Corvo::integration_connections()`](crate::Corvo::integration_connections).
///
/// A connection is tied to the environment of the key that created it.
pub struct IntegrationConnections<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> IntegrationConnections<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// List connections (`GET /integrations/connections`).
    pub async fn list(&self) -> Result<Vec<IntegrationConnection>, CorvoError> {
        let spec = RequestSpec::get("/integrations/connections");
        self.transport.request_json(&spec).await
    }

    /// Create a connection (`POST /integrations/connections`). The secrets in
    /// the answer are shown only once.
    pub async fn create(
        &self,
        params: &IntegrationConnectionCreateParams,
    ) -> Result<IntegrationConnectionCreated, CorvoError> {
        let spec = RequestSpec::post("/integrations/connections").json(params)?;
        self.transport.request_json(&spec).await
    }

    /// Get a connection (`GET /integrations/connections/:id`).
    pub async fn get(&self, id: &str) -> Result<IntegrationConnection, CorvoError> {
        self.simple_get(id, "").await
    }

    /// Check the credentials and identify the external account
    /// (`POST /integrations/connections/:id/validate`).
    pub async fn validate(&self, id: &str) -> Result<IntegrationConnection, CorvoError> {
        self.action(id, "validate").await
    }

    /// Replace the credentials, for example a rotated API key
    /// (`PUT /integrations/connections/:id/credentials`).
    pub async fn update_credentials(
        &self,
        id: &str,
        credentials: &BTreeMap<String, String>,
    ) -> Result<IntegrationConnection, CorvoError> {
        let spec = RequestSpec::put(&format!(
            "/integrations/connections/{}/credentials",
            encode(id)
        ))
        .json(&json!({ "credentials": credentials }))?;
        self.transport.request_json(&spec).await
    }

    /// Start issuing invoices automatically
    /// (`POST /integrations/connections/:id/activate`).
    pub async fn activate(&self, id: &str) -> Result<IntegrationConnection, CorvoError> {
        self.action(id, "activate").await
    }

    /// Stop issuing; events are still recorded
    /// (`POST /integrations/connections/:id/pause`).
    pub async fn pause(&self, id: &str) -> Result<IntegrationConnection, CorvoError> {
        self.action(id, "pause").await
    }

    /// Disconnect for good (`POST /integrations/connections/:id/disconnect`).
    pub async fn disconnect(&self, id: &str) -> Result<IntegrationConnection, CorvoError> {
        self.action(id, "disconnect").await
    }

    /// List the rules of a connection (`GET /integrations/connections/:id/rules`).
    pub async fn list_rules(&self, id: &str) -> Result<Vec<IntegrationRule>, CorvoError> {
        self.simple_get(id, "/rules").await
    }

    /// Replace the rules of a connection (`PUT /integrations/connections/:id/rules`).
    pub async fn replace_rules(
        &self,
        id: &str,
        rules: &[IntegrationRuleInput],
    ) -> Result<Vec<IntegrationRule>, CorvoError> {
        let spec = RequestSpec::put(&format!("/integrations/connections/{}/rules", encode(id)))
            .json(&json!({ "rules": rules }))?;
        self.transport.request_json(&spec).await
    }

    /// Recent events received for a connection
    /// (`GET /integrations/connections/:id/events`).
    pub async fn list_events(
        &self,
        id: &str,
        limit: Option<u32>,
    ) -> Result<Vec<IntegrationEvent>, CorvoError> {
        self.limited(id, "/events", limit).await
    }

    /// Recent operations (sales) and their invoice status
    /// (`GET /integrations/connections/:id/operations`).
    pub async fn list_operations(
        &self,
        id: &str,
        limit: Option<u32>,
    ) -> Result<Vec<IntegrationOperation>, CorvoError> {
        self.limited(id, "/operations", limit).await
    }

    async fn simple_get<T: serde::de::DeserializeOwned>(
        &self,
        id: &str,
        suffix: &str,
    ) -> Result<T, CorvoError> {
        let spec = RequestSpec::get(&format!(
            "/integrations/connections/{}{}",
            encode(id),
            suffix
        ));
        self.transport.request_json(&spec).await
    }

    async fn limited<T: serde::de::DeserializeOwned>(
        &self,
        id: &str,
        suffix: &str,
        limit: Option<u32>,
    ) -> Result<T, CorvoError> {
        let mut query = Vec::new();
        push_query(&mut query, "limit", limit.map(|l| l.to_string()));
        let spec = RequestSpec::get(&format!(
            "/integrations/connections/{}{}",
            encode(id),
            suffix
        ))
        .query(query);
        self.transport.request_json(&spec).await
    }

    async fn action(&self, id: &str, name: &str) -> Result<IntegrationConnection, CorvoError> {
        let spec = RequestSpec::post(&format!(
            "/integrations/connections/{}/{}",
            encode(id),
            name
        ));
        self.transport.request_json(&spec).await
    }
}

/// Operations (sales) of integrations, obtained via
/// [`Corvo::integration_operations()`](crate::Corvo::integration_operations).
pub struct IntegrationOperations<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> IntegrationOperations<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Fill in the customer of an operation waiting for it
    /// (`PATCH /integrations/operations/:id/customer`).
    pub async fn complete_customer(
        &self,
        id: &str,
        params: &CompleteCustomerParams,
    ) -> Result<IntegrationOperation, CorvoError> {
        let spec = RequestSpec::patch(&format!("/integrations/operations/{}/customer", encode(id)))
            .json(params)?;
        self.transport.request_json(&spec).await
    }

    /// Try to issue the invoice of an operation again
    /// (`POST /integrations/operations/:id/retry`).
    pub async fn retry(&self, id: &str) -> Result<IntegrationOperationRetry, CorvoError> {
        let spec = RequestSpec::post(&format!("/integrations/operations/{}/retry", encode(id)));
        self.transport.request_json(&spec).await
    }
}
