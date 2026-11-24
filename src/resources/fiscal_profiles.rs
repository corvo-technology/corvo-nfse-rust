//! Fiscal profiles and service tax profiles of a company: 3 + 5 ops.

use crate::error::CorvoError;
use crate::resources::companies::encode;
use crate::transport::{push_query, RequestSpec};
use crate::types::fiscal::{
    FiscalProfile, FiscalProfileUpsertParams, ServiceTaxProfile, ServiceTaxProfileCreateParams,
    ServiceTaxProfileUpdateParams,
};

/// Fiscal profiles resource, obtained via
/// [`Corvo::fiscal_profiles()`](crate::Corvo::fiscal_profiles).
pub struct FiscalProfiles<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> FiscalProfiles<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// The fiscal profile history of a company (`GET /companies/:id/fiscal-profile`).
    pub async fn list(&self, company_id: &str) -> Result<Vec<FiscalProfile>, CorvoError> {
        let spec = RequestSpec::get(&format!("/companies/{}/fiscal-profile", encode(company_id)));
        self.transport.request_json(&spec).await
    }

    /// Create a validity period; idempotent for an identical desired state
    /// (`PUT /companies/:id/fiscal-profile`).
    pub async fn upsert(
        &self,
        company_id: &str,
        params: &FiscalProfileUpsertParams,
    ) -> Result<FiscalProfile, CorvoError> {
        let spec = RequestSpec::put(&format!("/companies/{}/fiscal-profile", encode(company_id)))
            .json(params)?;
        self.transport.request_json(&spec).await
    }

    /// Approve a fiscal profile
    /// (`POST /companies/:id/fiscal-profile/:profile_id/approve`).
    pub async fn approve(
        &self,
        company_id: &str,
        profile_id: &str,
    ) -> Result<FiscalProfile, CorvoError> {
        let spec = RequestSpec::post(&format!(
            "/companies/{}/fiscal-profile/{}/approve",
            encode(company_id),
            encode(profile_id)
        ));
        self.transport.request_json(&spec).await
    }
}

/// Service tax profiles resource, obtained via
/// [`Corvo::service_tax_profiles()`](crate::Corvo::service_tax_profiles).
pub struct ServiceTaxProfiles<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> ServiceTaxProfiles<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Profiles effective on `date` (`YYYY-MM-DD`, default today)
    /// (`GET /companies/:id/service-tax-profiles`).
    pub async fn list(
        &self,
        company_id: &str,
        date: Option<&str>,
    ) -> Result<Vec<ServiceTaxProfile>, CorvoError> {
        let mut query = Vec::new();
        push_query(&mut query, "date", date.map(str::to_owned));
        let spec = RequestSpec::get(&format!(
            "/companies/{}/service-tax-profiles",
            encode(company_id)
        ))
        .query(query);
        self.transport.request_json(&spec).await
    }

    /// Create a profile in the catalog of the company
    /// (`POST /companies/:id/service-tax-profiles`).
    pub async fn create(
        &self,
        company_id: &str,
        params: &ServiceTaxProfileCreateParams,
    ) -> Result<ServiceTaxProfile, CorvoError> {
        let spec = RequestSpec::post(&format!(
            "/companies/{}/service-tax-profiles",
            encode(company_id)
        ))
        .json(params)?;
        self.transport.request_json(&spec).await
    }

    /// Update a profile
    /// (`PATCH /companies/:id/service-tax-profiles/:service_id`).
    pub async fn update(
        &self,
        company_id: &str,
        service_id: &str,
        params: &ServiceTaxProfileUpdateParams,
    ) -> Result<ServiceTaxProfile, CorvoError> {
        let spec = RequestSpec::patch(&format!(
            "/companies/{}/service-tax-profiles/{}",
            encode(company_id),
            encode(service_id)
        ))
        .json(params)?;
        self.transport.request_json(&spec).await
    }

    /// Delete an expired profile that was not approved
    /// (`DELETE /companies/:id/service-tax-profiles/:service_id`).
    pub async fn remove(&self, company_id: &str, service_id: &str) -> Result<(), CorvoError> {
        let spec = RequestSpec::delete(&format!(
            "/companies/{}/service-tax-profiles/{}",
            encode(company_id),
            encode(service_id)
        ));
        self.transport.request_unit(&spec).await
    }

    /// Approve the saved classification
    /// (`POST /companies/:id/service-tax-profiles/:service_id/approve`).
    pub async fn approve(
        &self,
        company_id: &str,
        service_id: &str,
    ) -> Result<ServiceTaxProfile, CorvoError> {
        let spec = RequestSpec::post(&format!(
            "/companies/{}/service-tax-profiles/{}/approve",
            encode(company_id),
            encode(service_id)
        ));
        self.transport.request_json(&spec).await
    }
}
