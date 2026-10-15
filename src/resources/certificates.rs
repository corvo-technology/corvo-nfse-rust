//! Digital certificates resource: 3 ops.

use reqwest::multipart::{Form, Part};

use crate::error::CorvoError;
use crate::resources::companies::encode;
use crate::transport::{push_query, RequestSpec, Unwrap};
use crate::types::certificate::{Certificate, CertificateUploadParams};
use crate::types::common::Paginated;

/// Certificates resource, obtained via
/// [`Corvo::certificates()`](crate::Corvo::certificates).
pub struct Certificates<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> Certificates<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Upload a PKCS#12 (`.p12`) certificate (`POST /companies/:id/certificates`).
    ///
    /// The multipart body is built for you: file part plus `password`.
    pub async fn upload(
        &self,
        company_id: &str,
        params: &CertificateUploadParams,
    ) -> Result<Certificate, CorvoError> {
        let file_name = params
            .file_name
            .clone()
            .unwrap_or_else(|| "certificate.p12".to_owned());
        let file_bytes = params.file.clone();
        let password = params.password.clone();
        let spec = RequestSpec::post(&format!("/companies/{}/certificates", encode(company_id)))
            .form(move || {
                let part = Part::bytes(file_bytes.clone())
                    .file_name(file_name.clone())
                    .mime_str("application/x-pkcs12")
                    .expect("valid x-pkcs12 mime");
                Form::new()
                    .part("file", part)
                    .text("password", password.clone())
            });
        self.transport.request_json(&spec).await
    }

    /// List a company's certificates (`GET /companies/:id/certificates`).
    pub async fn list(
        &self,
        company_id: &str,
        limit: Option<u16>,
        cursor: Option<&str>,
    ) -> Result<Paginated<Certificate>, CorvoError> {
        let mut query = Vec::new();
        push_query(&mut query, "limit", limit.map(|l| l.to_string()));
        push_query(&mut query, "cursor", cursor.map(str::to_owned));
        self.transport
            .list_page(
                &format!("/companies/{}/certificates", encode(company_id)),
                &query,
                Unwrap::None,
            )
            .await
    }

    /// Delete a certificate
    /// (`DELETE /companies/:id/certificates/:cert_id`). A `204` maps to `Ok(())`.
    pub async fn remove(&self, company_id: &str, cert_id: &str) -> Result<(), CorvoError> {
        let spec = RequestSpec::delete(&format!(
            "/companies/{}/certificates/{}",
            encode(company_id),
            encode(cert_id)
        ));
        self.transport
            .request_json::<serde::de::IgnoredAny>(&spec)
            .await
            .map(|_| ())
    }
}
