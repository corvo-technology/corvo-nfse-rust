//! Saved customer types.

use serde::{Deserialize, Serialize};

/// A customer saved from a previous invoice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedCustomer {
    /// CPF or CNPJ, digits only.
    pub tax_id: String,
    /// Name.
    pub name: String,
    /// E-mail, when known.
    pub email: Option<String>,
    /// When it was last used on an invoice (RFC 3339).
    pub last_used_at: String,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
    /// Last update timestamp (RFC 3339).
    pub updated_at: String,
}

/// Filters for [`Customers::search`](crate::Customers::search). The cursor is
/// the offset of the next page: pass it back untouched.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CustomerSearchParams {
    /// Text to look for in the name or tax id (up to 100 characters).
    pub search: Option<String>,
    /// Page size (1–1000, default 50).
    pub limit: Option<u16>,
    /// Opaque cursor from a previous page.
    pub cursor: Option<String>,
}

/// Correction of a saved customer. Only provided fields change.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomerUpdateParams {
    /// New name (up to 300 characters).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New e-mail; `Some(None)` clears it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
}
