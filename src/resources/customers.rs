//! Saved customers and CNPJ lookup: 5 + 1 ops.

use crate::error::CorvoError;
use crate::resources::companies::encode;
use crate::transport::{push_query, Paginator, RequestSpec, Unwrap};
use crate::types::cnpj::CnpjProfile;
use crate::types::common::Paginated;
use crate::types::customer::{CustomerSearchParams, CustomerUpdateParams, SavedCustomer};

/// Saved customers resource, obtained via
/// [`Corvo::customers()`](crate::Corvo::customers).
pub struct Customers<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> Customers<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// List saved customers, most recently used first (`GET /customers`).
    pub async fn search(
        &self,
        params: &CustomerSearchParams,
    ) -> Result<Paginated<SavedCustomer>, CorvoError> {
        self.transport
            .list_page("/customers", &search_query(params), Unwrap::None)
            .await
    }

    /// Stream every saved customer across pages (`GET /customers`).
    pub fn iterate(&self, params: &CustomerSearchParams) -> Paginator<'c, SavedCustomer> {
        Paginator::new(
            self.transport,
            "/customers",
            search_query(params),
            Unwrap::None,
            params.cursor.clone(),
        )
    }

    /// Get a saved customer by CPF or CNPJ, digits only (`GET /customers/:tax_id`).
    pub async fn get(&self, tax_id: &str) -> Result<SavedCustomer, CorvoError> {
        let spec = RequestSpec::get(&format!("/customers/{}", encode(tax_id)));
        self.transport.request_json(&spec).await
    }

    /// Correct the name or the e-mail of a saved customer
    /// (`PATCH /customers/:tax_id`).
    pub async fn update(
        &self,
        tax_id: &str,
        params: &CustomerUpdateParams,
    ) -> Result<SavedCustomer, CorvoError> {
        let spec = RequestSpec::patch(&format!("/customers/{}", encode(tax_id))).json(params)?;
        self.transport.request_json(&spec).await
    }

    /// Forget a saved customer; issued invoices are not changed
    /// (`DELETE /customers/:tax_id`).
    pub async fn remove(&self, tax_id: &str) -> Result<(), CorvoError> {
        let spec = RequestSpec::delete(&format!("/customers/{}", encode(tax_id)));
        self.transport.request_unit(&spec).await
    }
}

fn search_query(params: &CustomerSearchParams) -> Vec<(String, String)> {
    let mut query = Vec::new();
    push_query(&mut query, "search", params.search.clone());
    push_query(&mut query, "limit", params.limit.map(|l| l.to_string()));
    push_query(&mut query, "cursor", params.cursor.clone());
    query
}

/// CNPJ lookup, obtained via [`Corvo::cnpj()`](crate::Corvo::cnpj).
pub struct Cnpj<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> Cnpj<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Look up the registration data of a CNPJ (`GET /cnpj/:cnpj`).
    ///
    /// Fails with `CNPJ_INVALID` (bad digits), 404 (not found) or
    /// `CNPJ_LOOKUP_UNAVAILABLE` (the registry is down).
    pub async fn lookup(&self, cnpj: &str) -> Result<CnpjProfile, CorvoError> {
        let spec = RequestSpec::get(&format!("/cnpj/{}", encode(cnpj)));
        self.transport.request_json(&spec).await
    }
}
