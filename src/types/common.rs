//! Shared wire types: pagination envelopes.

use serde::{Deserialize, Serialize};

/// Issuance environment of a tenant or document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    /// Test environment (`corvo_test_` keys).
    #[serde(rename = "test")]
    Test,
    /// Production environment (`corvo_live_` keys).
    #[serde(rename = "live")]
    Live,
}

impl Environment {
    /// The exact string the API expects on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Test => "test",
            Environment::Live => "live",
        }
    }
}

/// Cursor pagination metadata returned alongside every list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    /// Page size that was applied (1–1000, default 50).
    pub limit: u16,
    /// Whether more pages exist after this one.
    pub has_more: bool,
    /// Opaque cursor for the next page; `None` when `has_more` is `false`.
    pub next_cursor: Option<String>,
}

/// A single page of results: the items plus their pagination metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paginated<T> {
    /// The items in this page.
    pub data: Vec<T>,
    /// Pagination metadata for the page.
    pub pagination: Pagination,
}
