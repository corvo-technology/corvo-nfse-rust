//! Companies (emitters) resource: 5 ops + `iterate`.

use crate::error::CorvoError;
use crate::transport::{push_query, Paginator, RequestSpec, Unwrap};
use crate::types::common::Paginated;
use crate::types::company::{Company, CompanyCreateParams, CompanyUpdateParams};

/// Filters for listing companies.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CompanyListParams {
    /// Page size (1–1000, default 50).
    pub limit: Option<u16>,
    /// Opaque cursor from a previous page.
    pub cursor: Option<String>,
}

/// Companies resource, obtained via [`Corvo::companies()`](crate::Corvo::companies).
pub struct Companies<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> Companies<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Create a company (`POST /companies`).
    pub async fn create(&self, params: &CompanyCreateParams) -> Result<Company, CorvoError> {
        let spec = RequestSpec::post("/companies").json(params)?;
        self.transport.request_json(&spec).await
    }

    /// List companies (`GET /companies`).
    pub async fn list(&self, params: CompanyListParams) -> Result<Paginated<Company>, CorvoError> {
        let mut query = Vec::new();
        push_query(&mut query, "limit", params.limit.map(|l| l.to_string()));
        push_query(&mut query, "cursor", params.cursor);
        self.transport.list_page("/companies", &query, Unwrap::Data)
            .await
    }

    /// Stream every company across pages (`GET /companies`).
    ///
    /// The first page failure surfaces as the first `Err` from the stream.
    pub fn iterate(&self, params: CompanyListParams) -> Paginator<'c, Company> {
        let mut query = Vec::new();
        push_query(&mut query, "limit", params.limit.map(|l| l.to_string()));
        Paginator::new(self.transport, "/companies", query, Unwrap::Data, params.cursor)
    }

    /// Fetch one company (`GET /companies/:id`).
    pub async fn get(&self, id: &str) -> Result<Company, CorvoError> {
        let spec = RequestSpec::get(&format!("/companies/{}", encode(id)));
        self.transport.request_json(&spec).await
    }

    /// Update a company (`PATCH /companies/:id`).
    pub async fn update(
        &self,
        id: &str,
        params: &CompanyUpdateParams,
    ) -> Result<Company, CorvoError> {
        let spec = RequestSpec::patch(&format!("/companies/{}", encode(id))).json(params)?;
        self.transport.request_json(&spec).await
    }

    /// Delete a company (`DELETE /companies/:id`). A `204` maps to `Ok(())`.
    pub async fn remove(&self, id: &str) -> Result<(), CorvoError> {
        let spec = RequestSpec::delete(&format!("/companies/{}", encode(id)));
        self.transport
            .request_json::<serde::de::IgnoredAny>(&spec)
            .await
            .map(|_| ())
    }
}

/// Percent-encode a path segment (mirrors the Node SDK's `encodeURIComponent`).
pub(crate) fn encode(segment: &str) -> String {
    let mut out = String::with_capacity(segment.len());
    for byte in segment.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}
