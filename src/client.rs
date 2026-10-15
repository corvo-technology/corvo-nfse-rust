//! The [`Corvo`] client entry point and its builder.

use std::env;
use std::time::Duration;

use crate::error::CorvoError;
use crate::resources::billing::Billing;
use crate::resources::certificates::Certificates;
use crate::resources::companies::Companies;
use crate::resources::credits::Credits;
use crate::resources::health::Health;
use crate::resources::invoices::Invoices;
use crate::resources::webhooks::Webhooks;
use crate::transport::Transport;

/// Default public API base URL (production).
pub const DEFAULT_BASE_URL: &str = "https://api.corvo.tech";

/// The Corvo NFS-e API client.
///
/// Composes resource namespaces over a shared HTTP transport:
/// `corvo.invoices()`, `corvo.companies()`, and so on.
///
/// ```no_run
/// # async fn run() -> Result<(), corvo_nfse::CorvoError> {
/// use corvo_nfse::Corvo;
///
/// let corvo = Corvo::builder()
///     .api_key("corvo_test_...")
///     .base_url("https://api.corvo.tech")
///     .build()?;
/// let companies = corvo.companies().list(Default::default()).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Corvo {
    transport: Transport,
}

impl Corvo {
    /// Create a client from an API key and base URL.
    ///
    /// Environment fallbacks (`CORVO_API_KEY`, `CORVO_BASE_URL`) are available
    /// through [`Corvo::builder`] / [`Corvo::from_env`]; this constructor takes
    /// both values explicitly.
    pub fn new(
        api_key: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Result<Self, CorvoError> {
        Self::builder().api_key(api_key).base_url(base_url).build()
    }

    /// Create a client configured entirely from the environment
    /// (`CORVO_API_KEY`, `CORVO_BASE_URL`).
    ///
    /// Fails with [`CorvoError::Config`] when `CORVO_API_KEY` is not set.
    pub fn from_env() -> Result<Self, CorvoError> {
        Self::builder().build()
    }

    /// Start building a client with explicit options.
    pub fn builder() -> CorvoBuilder {
        CorvoBuilder::default()
    }

    /// Companies (emitters) resource.
    pub fn companies(&self) -> Companies<'_> {
        Companies::new(&self.transport)
    }

    /// Digital certificates resource.
    pub fn certificates(&self) -> Certificates<'_> {
        Certificates::new(&self.transport)
    }

    /// Invoices resource.
    pub fn invoices(&self) -> Invoices<'_> {
        Invoices::new(&self.transport)
    }

    /// Webhooks resource (including deliveries).
    pub fn webhooks(&self) -> Webhooks<'_> {
        Webhooks::new(&self.transport)
    }

    /// Credits resource.
    pub fn credits(&self) -> Credits<'_> {
        Credits::new(&self.transport)
    }

    /// Billing resource.
    pub fn billing(&self) -> Billing<'_> {
        Billing::new(&self.transport)
    }

    /// Health checks (no API key required).
    pub fn health(&self) -> Health<'_> {
        Health::new(&self.transport)
    }
}

/// Builder for [`Corvo`], resolving defaults from the environment.
///
/// Resolution order: explicit option, then environment variable, then default.
/// `api_key` falls back to `CORVO_API_KEY` (required), `base_url` to
/// `CORVO_BASE_URL` (default [`DEFAULT_BASE_URL`]), and `prefix` to `v1`.
#[derive(Debug, Default, Clone)]
pub struct CorvoBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    prefix: Option<String>,
    timeout: Option<Duration>,
    allow_insecure_http: bool,
}

impl CorvoBuilder {
    /// Set the API key (`corvo_test_…` / `corvo_live_…`).
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set the base URL (self-hosted installs).
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Set the API prefix (default `v1`).
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Set the request timeout (default 30 seconds).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Allow `http://` base URLs outside loopback/private hosts (default
    /// `false`). Never enable this in production.
    pub fn allow_insecure_http(mut self, allow: bool) -> Self {
        self.allow_insecure_http = allow;
        self
    }

    /// Build the client, resolving env fallbacks. Returns
    /// [`CorvoError::Config`] when no API key is available, or
    /// [`CorvoError::InvalidUrl`] when the base URL is unusable.
    pub fn build(self) -> Result<Corvo, CorvoError> {
        let api_key = self
            .api_key
            .filter(|s| !s.is_empty())
            .or_else(|| env::var("CORVO_API_KEY").ok().filter(|s| !s.is_empty()))
            .ok_or_else(|| {
                CorvoError::Config(
                    "api_key is required — pass .api_key(...) or set CORVO_API_KEY".to_owned(),
                )
            })?;
        let base_url = self
            .base_url
            .filter(|s| !s.is_empty())
            .or_else(|| env::var("CORVO_BASE_URL").ok().filter(|s| !s.is_empty()))
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_owned());
        let transport = Transport::new(
            api_key,
            &base_url,
            self.prefix.as_deref(),
            self.timeout.map(|t| t.as_millis() as u64),
            self.allow_insecure_http,
        )?;
        Ok(Corvo { transport })
    }
}
