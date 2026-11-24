//! CNPJ lookup types.

use serde::{Deserialize, Serialize};

/// A postal address as the registry returns it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostalAddress {
    /// City.
    pub city: String,
    /// Complement, when there is one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complement: Option<String>,
    /// District (bairro).
    pub district: String,
    /// Street number.
    pub number: String,
    /// State (UF, 2 letters).
    pub state: String,
    /// Street.
    pub street: String,
    /// CEP.
    pub zip: String,
}

/// Registration data of a CNPJ (`GET /cnpj/{cnpj}`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CnpjProfile {
    /// Whether the company is active.
    pub active: bool,
    /// Registered address.
    pub address: PostalAddress,
    /// Registered legal name (razão social).
    pub legal_name: String,
    /// Whether the company is a MEI, when known.
    #[serde(default)]
    pub mei: Option<bool>,
    /// IBGE municipality code, when known.
    #[serde(default)]
    pub municipality_code: Option<String>,
    /// Whether the company is in Simples Nacional, when known.
    #[serde(default)]
    pub simples: Option<bool>,
    /// Registration situation (for example `ATIVA`).
    pub situation: String,
    /// Trade name (nome fantasia); empty when there is none.
    #[serde(default)]
    pub trade_name: String,
}
