//! Monthly invoice package (ZIP for the accountant): 2 ops.

use crate::error::CorvoError;
use crate::resources::companies::encode;
use crate::transport::{push_query, DownloadedFile, RequestSpec};
use crate::types::invoice::{PackagePart, PackageSummary};

/// Invoice package resource, obtained via
/// [`Corvo::invoice_package()`](crate::Corvo::invoice_package).
pub struct InvoicePackage<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> InvoicePackage<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Count and total of the package of `month` (`YYYY-MM`), without building
    /// it (`GET /companies/:company_id/invoice-package/summary`).
    pub async fn summary(
        &self,
        company_id: &str,
        month: &str,
    ) -> Result<PackageSummary, CorvoError> {
        let mut query = Vec::new();
        push_query(&mut query, "month", Some(month));
        let spec = RequestSpec::get(&format!(
            "/companies/{}/invoice-package/summary",
            encode(company_id)
        ))
        .query(query);
        self.transport.request_json(&spec).await
    }

    /// Download the package of `month` (`YYYY-MM`): a ZIP with `xml/`, `pdf/`
    /// and `resumo.csv` (`GET /companies/:company_id/invoice-package`).
    ///
    /// `include` limits the parts; `None` includes both.
    pub async fn download(
        &self,
        company_id: &str,
        month: &str,
        include: Option<&[PackagePart]>,
    ) -> Result<DownloadedFile, CorvoError> {
        let mut query = Vec::new();
        push_query(&mut query, "month", Some(month));
        if let Some(parts) = include.filter(|p| !p.is_empty()) {
            let joined = parts
                .iter()
                .map(PackagePart::as_str)
                .collect::<Vec<_>>()
                .join(",");
            push_query(&mut query, "include", Some(joined));
        }
        let spec = RequestSpec::get(&format!(
            "/companies/{}/invoice-package",
            encode(company_id)
        ))
        .query(query)
        .accept("application/zip");
        self.transport.request_raw(&spec).await
    }
}
