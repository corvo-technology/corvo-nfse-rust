//! Favorite services of a company: 3 ops.

use crate::error::CorvoError;
use crate::resources::companies::encode;
use crate::transport::RequestSpec;
use crate::types::common::Deleted;
use crate::types::service_favorite::{ServiceFavorite, ServiceFavoriteCreateParams};

/// Favorite services resource, obtained via
/// [`Corvo::service_favorites()`](crate::Corvo::service_favorites).
pub struct ServiceFavorites<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> ServiceFavorites<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// List the favorite services of a company
    /// (`GET /companies/:company_id/service-favorites`).
    pub async fn list(&self, company_id: &str) -> Result<Vec<ServiceFavorite>, CorvoError> {
        let spec = RequestSpec::get(&format!(
            "/companies/{}/service-favorites",
            encode(company_id)
        ));
        self.transport.request_json(&spec).await
    }

    /// Save a favorite service (`POST /companies/:company_id/service-favorites`).
    pub async fn create(
        &self,
        company_id: &str,
        params: &ServiceFavoriteCreateParams,
    ) -> Result<ServiceFavorite, CorvoError> {
        let spec = RequestSpec::post(&format!(
            "/companies/{}/service-favorites",
            encode(company_id)
        ))
        .json(params)?;
        self.transport.request_json(&spec).await
    }

    /// Remove a favorite service
    /// (`DELETE /companies/:company_id/service-favorites/:favorite_id`).
    pub async fn remove(&self, company_id: &str, favorite_id: &str) -> Result<Deleted, CorvoError> {
        let spec = RequestSpec::delete(&format!(
            "/companies/{}/service-favorites/{}",
            encode(company_id),
            encode(favorite_id)
        ));
        self.transport.request_json(&spec).await
    }
}
