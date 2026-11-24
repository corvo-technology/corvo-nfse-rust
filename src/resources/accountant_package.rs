//! Accountant package of a company: 4 ops.

use serde_json::json;

use crate::error::CorvoError;
use crate::resources::companies::encode;
use crate::transport::RequestSpec;
use crate::types::accountant::{
    AccountantPackageUpdateParams, AccountantSend, AccountantSendResult, AccountantSettings,
    AccountantSettingsUpdate,
};

/// Accountant package resource, obtained via
/// [`Corvo::accountant_package()`](crate::Corvo::accountant_package).
pub struct AccountantPackage<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> AccountantPackage<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// The accountant e-mail of the monthly package
    /// (`GET /companies/:company_id/accountant-package`).
    pub async fn get(&self, company_id: &str) -> Result<AccountantSettings, CorvoError> {
        let spec = RequestSpec::get(&format!(
            "/companies/{}/accountant-package",
            encode(company_id)
        ));
        self.transport.request_json(&spec).await
    }

    /// Set the accountant e-mail; `None` turns the monthly send off
    /// (`PUT /companies/:company_id/accountant-package`).
    pub async fn update(
        &self,
        company_id: &str,
        accountant_email: Option<&str>,
    ) -> Result<AccountantSettingsUpdate, CorvoError> {
        let params = AccountantPackageUpdateParams {
            accountant_email: accountant_email.map(str::to_owned),
        };
        let spec = RequestSpec::put(&format!(
            "/companies/{}/accountant-package",
            encode(company_id)
        ))
        .json(&params)?;
        self.transport.request_json(&spec).await
    }

    /// Send the package of `month` (`YYYY-MM`) to the accountant now
    /// (`POST /companies/:company_id/accountant-package/send`).
    ///
    /// A package over the size limit fails with `PACKAGE_TOO_LARGE`.
    pub async fn send(
        &self,
        company_id: &str,
        month: &str,
    ) -> Result<AccountantSendResult, CorvoError> {
        let spec = RequestSpec::post(&format!(
            "/companies/{}/accountant-package/send",
            encode(company_id)
        ))
        .json(&json!({ "month": month }))?;
        self.transport.request_json(&spec).await
    }

    /// The last 12 attempts to send the package
    /// (`GET /companies/:company_id/accountant-package/sends`).
    pub async fn list_sends(&self, company_id: &str) -> Result<Vec<AccountantSend>, CorvoError> {
        let spec = RequestSpec::get(&format!(
            "/companies/{}/accountant-package/sends",
            encode(company_id)
        ));
        self.transport.request_json(&spec).await
    }
}
