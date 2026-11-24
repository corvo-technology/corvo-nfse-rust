//! National NFS-e portal status: 1 op.

use crate::error::CorvoError;
use crate::transport::RequestSpec;
use crate::types::invoice::PortalStatus;

/// Fiscal portal resource, obtained via
/// [`Corvo::fiscal_portal()`](crate::Corvo::fiscal_portal).
pub struct FiscalPortal<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> FiscalPortal<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Whether the national NFS-e portal is answering for this environment
    /// (`GET /fiscal-portal/status`).
    pub async fn status(&self) -> Result<PortalStatus, CorvoError> {
        let spec = RequestSpec::get("/fiscal-portal/status");
        self.transport.request_json(&spec).await
    }
}
