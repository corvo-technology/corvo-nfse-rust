//! DPS numbering of a company: 2 ops.

use crate::error::CorvoError;
use crate::resources::companies::encode;
use crate::transport::RequestSpec;
use crate::types::fiscal::{DpsSequence, DpsSequenceSetParams};

/// DPS numbering resource, obtained via
/// [`Corvo::dps_sequence()`](crate::Corvo::dps_sequence).
pub struct DpsSequenceResource<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> DpsSequenceResource<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// The DPS numbering of a company (`GET /companies/:id/dps-sequence`).
    pub async fn get(&self, company_id: &str) -> Result<DpsSequence, CorvoError> {
        let spec = RequestSpec::get(&format!("/companies/{}/dps-sequence", encode(company_id)));
        self.transport.request_json(&spec).await
    }

    /// Continue the numbering from the last note issued elsewhere
    /// (`PUT /companies/:id/dps-sequence`).
    ///
    /// A number below the current one fails with `DPS_NUMBER_BELOW_CURRENT`.
    pub async fn set(
        &self,
        company_id: &str,
        params: &DpsSequenceSetParams,
    ) -> Result<DpsSequence, CorvoError> {
        let spec = RequestSpec::put(&format!("/companies/{}/dps-sequence", encode(company_id)))
            .json(params)?;
        self.transport.request_json(&spec).await
    }
}
