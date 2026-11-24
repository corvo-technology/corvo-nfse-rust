//! Favorite service types.

use serde::{Deserialize, Serialize};

/// A favorite service saved for a company.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceFavorite {
    /// Unique id.
    pub id: String,
    /// Owning tenant id.
    pub tenant_id: String,
    /// Company it belongs to.
    pub company_id: String,
    /// Short name (up to 80 characters).
    pub label: String,
    /// National service code (6 digits).
    pub service_code: String,
    /// Service description.
    pub description: String,
    /// Usual amount in centavos.
    pub amount_cents: Option<i64>,
    /// IBGE municipality code.
    pub municipality_code: Option<String>,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
}

/// Parameters for saving a favorite service.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceFavoriteCreateParams {
    /// Short name (1 to 80 characters).
    pub label: String,
    /// National service code (6 digits).
    pub service_code: String,
    /// Service description (1 to 2000 characters).
    pub description: String,
    /// Usual amount in centavos (at least 1).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i64>,
    /// IBGE municipality code (7 digits).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub municipality_code: Option<String>,
}
