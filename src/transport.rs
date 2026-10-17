//! HTTP transport: URL building, auth headers, envelope unwrapping, error
//! mapping, and cursor pagination.
//!
//! `Transport` itself is internal; the public pieces are [`DownloadedFile`]
//! and [`Paginator`], which appear in resource method signatures.

use std::collections::VecDeque;
use std::time::Duration;

use futures_util::{Stream, StreamExt};
use reqwest::header::{HeaderMap, ACCEPT, RETRY_AFTER, USER_AGENT};
use reqwest::multipart::Form;
use reqwest::{Client, Response, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;

use crate::error::{CorvoError, ErrorDetail};
use crate::types::common::{Paginated, Pagination};

/// Envelope unwrapping mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Unwrap {
    /// Return `body.data` (or `null` when missing).
    Data,
    /// Return the whole body untouched.
    None,
}

/// HTTP methods supported by the API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Method {
    /// GET.
    Get,
    /// POST.
    Post,
    /// PATCH.
    Patch,
    /// DELETE.
    Delete,
}

impl Method {
    fn as_reqwest(self) -> reqwest::Method {
        match self {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Patch => reqwest::Method::PATCH,
            Method::Delete => reqwest::Method::DELETE,
        }
    }
}

/// A fully-specified outgoing request.
pub(crate) struct RequestSpec {
    method: Method,
    path: String,
    query: Vec<(String, String)>,
    json: Option<Value>,
    form: Option<Box<dyn Fn() -> Form + Send + Sync>>,
    idempotency_key: Option<String>,
    unwrap: Unwrap,
}

impl std::fmt::Debug for RequestSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestSpec")
            .field("method", &self.method)
            .field("path", &self.path)
            .field("query", &self.query)
            .field("json", &self.json)
            .field("has_form", &self.form.is_some())
            .field("idempotency_key", &self.idempotency_key)
            .field("unwrap", &self.unwrap)
            .finish()
    }
}

impl RequestSpec {
    fn new(method: Method, path: &str) -> Self {
        Self {
            method,
            path: path.to_owned(),
            query: Vec::new(),
            json: None,
            form: None,
            idempotency_key: None,
            unwrap: Unwrap::Data,
        }
    }

    pub(crate) fn get(path: &str) -> Self {
        Self::new(Method::Get, path)
    }

    pub(crate) fn post(path: &str) -> Self {
        Self::new(Method::Post, path)
    }

    pub(crate) fn patch(path: &str) -> Self {
        Self::new(Method::Patch, path)
    }

    pub(crate) fn delete(path: &str) -> Self {
        Self::new(Method::Delete, path)
    }

    pub(crate) fn query(mut self, pairs: Vec<(String, String)>) -> Self {
        self.query = pairs;
        self
    }

    pub(crate) fn json<T: serde::Serialize>(mut self, value: &T) -> Result<Self, CorvoError> {
        self.json =
            Some(serde_json::to_value(value).map_err(|e| {
                CorvoError::Config(format!("failed to serialize request body: {e}"))
            })?);
        Ok(self)
    }

    pub(crate) fn form(mut self, build: impl Fn() -> Form + Send + Sync + 'static) -> Self {
        self.form = Some(Box::new(build));
        self
    }

    pub(crate) fn idempotency_key(mut self, key: Option<String>) -> Self {
        self.idempotency_key = key;
        self
    }

    pub(crate) fn unwrap_mode(mut self, mode: Unwrap) -> Self {
        self.unwrap = mode;
        self
    }
}

/// Push `(key, value)` onto `query` when `value` is `Some`.
pub(crate) fn push_query(
    query: &mut Vec<(String, String)>,
    key: &str,
    value: Option<impl Into<String>>,
) {
    if let Some(v) = value {
        query.push((key.to_owned(), v.into()));
    }
}

/// A downloaded binary document (PDF/XML).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadedFile {
    /// Raw bytes of the document.
    pub bytes: Vec<u8>,
    /// MIME type reported by the server, when present.
    pub mime_type: Option<String>,
    /// File name suggested by the server (`Content-Disposition`), when present.
    pub file_name: Option<String>,
}

/// A page of a list endpoint, raw on the wire.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawPage {
    data: Vec<Value>,
    pagination: Pagination,
}

/// The shared HTTP client configuration.
#[derive(Debug)]
pub(crate) struct Transport {
    client: Client,
    api_key: String,
    base_url: Url,
    prefix: String,
}

impl Transport {
    pub(crate) fn new(
        api_key: String,
        base_url: &str,
        prefix: Option<&str>,
        timeout_ms: Option<u64>,
        allow_insecure_http: bool,
    ) -> Result<Self, CorvoError> {
        let trimmed = base_url.trim_end_matches('/');
        let url = Url::parse(trimmed).map_err(|_| CorvoError::InvalidUrl(trimmed.to_owned()))?;
        match url.scheme() {
            "https" => {}
            "http" => {
                let host = url.host_str().unwrap_or_default();
                if !allow_insecure_http && !is_private_host(host) {
                    return Err(CorvoError::InvalidUrl(format!(
                        "http:// base URLs are only allowed for loopback/private hosts (got {host}); \
                         pass allow_insecure_http(true) to override"
                    )));
                }
            }
            other => {
                return Err(CorvoError::InvalidUrl(format!(
                    "unsupported scheme {other:?} in base URL {trimmed}"
                )));
            }
        }
        let timeout = Duration::from_millis(timeout_ms.unwrap_or(30_000));
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| CorvoError::Network(format!("failed to build HTTP client: {e}")))?;
        Ok(Self {
            client,
            api_key,
            base_url: url,
            prefix: prefix.unwrap_or("v1").trim_matches('/').to_owned(),
        })
    }

    fn build_url(&self, path: &str, query: &[(String, String)]) -> Result<Url, CorvoError> {
        let mut url = self.base_url.clone();
        {
            let mut segments = url.path_segments_mut().map_err(|_| {
                CorvoError::InvalidUrl(format!("base URL cannot be a base: {}", self.base_url))
            })?;
            segments.pop_if_empty();
            if !self.prefix.is_empty() {
                for segment in self.prefix.split('/').filter(|s| !s.is_empty()) {
                    segments.push(segment);
                }
            }
            for segment in path.split('/').filter(|s| !s.is_empty()) {
                segments.push(segment);
            }
        }
        if !query.is_empty() {
            url.query_pairs_mut()
                .extend_pairs(query.iter().map(|(k, v)| (k.as_str(), v.as_str())));
        }
        Ok(url)
    }

    async fn send(&self, spec: &RequestSpec) -> Result<Response, CorvoError> {
        let url = self.build_url(&spec.path, &spec.query)?;
        let mut request = self
            .client
            .request(spec.method.as_reqwest(), url)
            .header("X-API-Key", &self.api_key)
            .header(ACCEPT, "application/json")
            .header(
                USER_AGENT,
                concat!("corvo-nfse-rust/", env!("CARGO_PKG_VERSION")),
            );
        if let Some(key) = &spec.idempotency_key {
            request = request.header("Idempotency-Key", key);
        }
        if let Some(build) = &spec.form {
            request = request.multipart(build());
        } else if let Some(json) = &spec.json {
            request = request.json(json);
        }
        request.send().await.map_err(map_reqwest_error)
    }

    /// Perform a request and deserialize the (unwrapped) JSON envelope.
    pub(crate) async fn request_json<T: DeserializeOwned>(
        &self,
        spec: &RequestSpec,
    ) -> Result<T, CorvoError> {
        let response = self.send(spec).await?;
        let status = response.status();
        let request_id = header_str(response.headers(), "x-request-id").map(str::to_owned);
        if !status.is_success() {
            return Err(to_error(status, response, request_id).await);
        }
        let body = response.bytes().await.map_err(map_reqwest_error)?;
        if status == StatusCode::NO_CONTENT || body.is_empty() {
            return serde_json::from_value(Value::Null).map_err(|_| {
                invalid_response(
                    status.as_u16(),
                    "empty response body where a value was expected".to_owned(),
                    request_id.clone(),
                )
            });
        }
        let raw: Value = serde_json::from_slice(&body).map_err(|_| {
            invalid_response(
                status.as_u16(),
                "response body was not valid JSON".to_owned(),
                request_id.clone(),
            )
        })?;
        let value = unwrap_envelope(raw, spec.unwrap);
        T::deserialize(value).map_err(|e| {
            invalid_response(
                status.as_u16(),
                format!("failed to decode response body: {e}"),
                request_id,
            )
        })
    }

    /// Perform a request and return the raw bytes (PDF/XML downloads).
    pub(crate) async fn request_raw(
        &self,
        spec: &RequestSpec,
    ) -> Result<DownloadedFile, CorvoError> {
        let response = self.send(spec).await?;
        let status = response.status();
        let request_id = header_str(response.headers(), "x-request-id").map(str::to_owned);
        if !status.is_success() {
            return Err(to_error(status, response, request_id).await);
        }
        let mime_type = header_str(response.headers(), "content-type")
            .map(|v| v.split(';').next().unwrap_or(v).trim().to_owned());
        let file_name = header_str(response.headers(), "content-disposition")
            .and_then(parse_content_disposition_filename);
        let bytes = response.bytes().await.map_err(map_reqwest_error)?.to_vec();
        Ok(DownloadedFile {
            bytes,
            mime_type,
            file_name,
        })
    }

    /// Fetch one page of a list endpoint, decoding each item.
    pub(crate) async fn list_page<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(String, String)],
        item_unwrap: Unwrap,
    ) -> Result<Paginated<T>, CorvoError> {
        let spec = RequestSpec::get(path)
            .query(query.to_vec())
            .unwrap_mode(Unwrap::None);
        let page: RawPage = self.request_json(&spec).await?;
        let mut items = Vec::with_capacity(page.data.len());
        for raw in page.data {
            let value = unwrap_envelope(raw, item_unwrap);
            items.push(T::deserialize(value).map_err(|e| {
                invalid_response(200, format!("failed to decode list item: {e}"), None)
            })?);
        }
        Ok(Paginated {
            data: items,
            pagination: page.pagination,
        })
    }
}

fn map_reqwest_error(e: reqwest::Error) -> CorvoError {
    if e.is_timeout() {
        CorvoError::Timeout
    } else {
        CorvoError::Network(e.to_string())
    }
}

fn invalid_response(status: u16, message: String, request_id: Option<String>) -> CorvoError {
    CorvoError::Api {
        status,
        code: "invalid_response".to_owned(),
        message,
        details: Vec::new(),
        request_id,
        retry_after: None,
    }
}

fn unwrap_envelope(value: Value, mode: Unwrap) -> Value {
    match mode {
        Unwrap::None => value,
        Unwrap::Data => value.get("data").cloned().unwrap_or(Value::Null),
    }
}

/// Map a non-2xx response to [`CorvoError::Api`].
async fn to_error(
    status: StatusCode,
    response: Response,
    request_id: Option<String>,
) -> CorvoError {
    let retry_after_header = header_str(response.headers(), RETRY_AFTER.as_str())
        .and_then(|v| v.trim().parse::<u64>().ok());
    let body = response.text().await.unwrap_or_default();
    let parsed: Value = serde_json::from_str(&body).unwrap_or(Value::Null);

    let code = parsed
        .get("code")
        .and_then(|c| c.as_str())
        .map(str::to_owned)
        .unwrap_or_else(|| format!("http_{}", status.as_u16()));

    let message = match parsed.get("message") {
        Some(Value::String(s)) if !s.is_empty() => s.clone(),
        Some(Value::Array(items)) => {
            let parts: Vec<&str> = items.iter().filter_map(|i| i.as_str()).collect();
            if parts.is_empty() {
                format!("Request failed with status {}", status.as_u16())
            } else {
                parts.join("; ")
            }
        }
        _ => match parsed.get("error").and_then(|e| e.as_str()) {
            Some(e) if !e.is_empty() => e.to_owned(),
            _ => format!("Request failed with status {}", status.as_u16()),
        },
    };

    let details: Vec<ErrorDetail> = parsed
        .get("details")
        .and_then(|d| d.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|d| serde_json::from_value::<ErrorDetail>(d.clone()).ok())
                .collect()
        })
        .unwrap_or_default();

    let request_id = parsed
        .get("requestId")
        .and_then(|r| r.as_str())
        .map(str::to_owned)
        .or(request_id);

    let retry_after = parsed
        .get("retryAfter")
        .and_then(|r| r.as_u64())
        .or(retry_after_header);

    CorvoError::Api {
        status: status.as_u16(),
        code,
        message,
        details,
        request_id,
        retry_after,
    }
}

fn header_str<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name).and_then(|v| v.to_str().ok())
}

/// Parse `filename="foo.pdf"` out of a `Content-Disposition` header.
fn parse_content_disposition_filename(value: &str) -> Option<String> {
    let lower = value.to_ascii_lowercase();
    let idx = lower.find("filename=")?;
    let rest = &value[idx + "filename=".len()..];
    let rest = rest.trim_start();
    let rest = rest.strip_prefix('"').unwrap_or(rest);
    let name = if rest.starts_with('"') {
        ""
    } else {
        match rest.find('"') {
            Some(end) => &rest[..end],
            None => rest.split(';').next().unwrap_or(rest).trim(),
        }
    };
    if name.is_empty() {
        None
    } else {
        Some(name.to_owned())
    }
}

/// Whether a hostname is loopback/private (mirrors the Node SDK).
fn is_private_host(host: &str) -> bool {
    let host = host.to_ascii_lowercase();
    if host == "localhost" || host == "[::1]" || host == "::1" || host.ends_with(".localhost") {
        return true;
    }
    let octets: Vec<&str> = host.split('.').collect();
    if octets.len() == 4 {
        if let Ok(nums) = octets
            .iter()
            .map(|o| o.parse::<u8>())
            .collect::<Result<Vec<u8>, _>>()
        {
            let [a, b] = [nums[0], nums[1]];
            return a == 127
                || a == 10
                || a == 0
                || (a == 192 && b == 168)
                || (a == 172 && (16..=31).contains(&b))
                || (a == 169 && b == 254);
        }
    }
    false
}

/// A cursor-based paginator over a list endpoint.
///
/// Created by each resource's `iterate` method. Drives it as a
/// [`Stream`] of `Result<T, CorvoError>` — the first page failure surfaces as
/// an `Err` from `next()`, and no further pages are fetched after an error or
/// after `has_more` turns false. `collect()` is a convenience for gathering
/// every remaining item into a `Vec`.
pub struct Paginator<'a, T> {
    transport: &'a Transport,
    path: String,
    query: Vec<(String, String)>,
    item_unwrap: Unwrap,
    cursor: Option<String>,
    buffer: VecDeque<T>,
    done: bool,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T> Paginator<'a, T> {
    pub(crate) fn new(
        transport: &'a Transport,
        path: &str,
        query: Vec<(String, String)>,
        item_unwrap: Unwrap,
        initial_cursor: Option<String>,
    ) -> Self {
        Self {
            transport,
            path: path.to_owned(),
            query,
            item_unwrap,
            cursor: initial_cursor,
            buffer: VecDeque::new(),
            done: false,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, T: DeserializeOwned + 'a> Paginator<'a, T> {
    /// Fetch every remaining item, stopping at the first page failure.
    pub async fn collect(self) -> Result<Vec<T>, CorvoError> {
        let mut out = Vec::new();
        let mut stream = Box::pin(self.stream());
        while let Some(item) = stream.next().await {
            out.push(item?);
        }
        Ok(out)
    }

    /// Stream the items page by page. Page failures surface as `Err` items.
    pub fn stream(self) -> impl Stream<Item = Result<T, CorvoError>> + 'a {
        futures_util::stream::try_unfold(self, |mut state: Paginator<'a, T>| async move {
            loop {
                if let Some(item) = state.buffer.pop_front() {
                    return Ok(Some((item, state)));
                }
                if state.done {
                    return Ok(None);
                }
                let mut query = state.query.clone();
                if let Some(cursor) = state.cursor.clone() {
                    query.push(("cursor".to_owned(), cursor));
                }
                let page: Paginated<T> = state
                    .transport
                    .list_page(&state.path, &query, state.item_unwrap)
                    .await?;
                state.cursor = page.pagination.next_cursor.clone();
                state.done = !page.pagination.has_more || state.cursor.is_none();
                state.buffer = page.data.into();
            }
        })
    }
}
