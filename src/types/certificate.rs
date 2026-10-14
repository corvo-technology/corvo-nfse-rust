//! Digital certificate (e-CNPJ A1) types.

use serde::{Deserialize, Serialize};

/// A stored PKCS#12 certificate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Certificate {
    /// Unique id.
    pub id: String,
    /// Company the certificate belongs to.
    pub company_id: String,
    /// Certificate serial number.
    pub serial_number: String,
    /// Issuing authority.
    pub issuer: String,
    /// Subject (holder) distinguished name.
    pub subject: String,
    /// Certificate fingerprint.
    pub fingerprint: String,
    /// Validity start (RFC 3339).
    pub valid_from: String,
    /// Validity end (RFC 3339).
    pub valid_until: String,
    /// Lifecycle status.
    pub status: String,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
}

/// Input for uploading a PKCS#12 certificate.
///
/// Build this with [`CertificateUploadParams::new`], which infers the file name
/// from the bytes' origin. The SDK sends it as `multipart/form-data`; you never
/// touch the multipart encoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateUploadParams {
    /// Raw `.p12`/`.pfx` bytes (max 50 KiB).
    pub file: Vec<u8>,
    /// File name reported to the server; defaults to `certificate.p12`.
    pub file_name: Option<String>,
    /// Password protecting the PKCS#12 bundle.
    pub password: String,
}

impl CertificateUploadParams {
    /// Create upload params from raw `.p12` bytes and the bundle password.
    pub fn new(file: impl Into<Vec<u8>>, password: impl Into<String>) -> Self {
        Self {
            file: file.into(),
            file_name: None,
            password: password.into(),
        }
    }

    /// Override the reported file name.
    pub fn file_name(mut self, name: impl Into<String>) -> Self {
        self.file_name = Some(name.into());
        self
    }
}
