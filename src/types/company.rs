//! Company (emitter) types.

use serde::{Deserialize, Serialize};

/// Brazilian tax regime of a company.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxRegime {
    /// Microempreendedor Individual.
    #[serde(rename = "MEI")]
    Mei,
    /// Simples Nacional.
    #[serde(rename = "Simples Nacional")]
    SimplesNacional,
    /// Lucro Presumido.
    #[serde(rename = "Lucro Presumido")]
    LucroPresumido,
    /// Lucro Real.
    #[serde(rename = "Lucro Real")]
    LucroReal,
}

impl TaxRegime {
    /// The exact string the API expects on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            TaxRegime::Mei => "MEI",
            TaxRegime::SimplesNacional => "Simples Nacional",
            TaxRegime::LucroPresumido => "Lucro Presumido",
            TaxRegime::LucroReal => "Lucro Real",
        }
    }
}

/// Postal address input for a company.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyAddressInput {
    /// Street name.
    pub street: String,
    /// Street number; optional (some municipalities omit it).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// Complement (suite, floor, …).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complement: Option<String>,
    /// District (bairro).
    pub district: String,
    /// City name.
    pub city: String,
    /// Two-letter state code.
    pub state: String,
    /// CEP (zip code).
    pub zip: String,
}

/// Parameters for creating a company.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyCreateParams {
    /// Registered legal name (razão social).
    pub legal_name: String,
    /// CNPJ tax id, digits only.
    pub tax_id: String,
    /// Tax regime.
    pub tax_regime: TaxRegime,
    /// Trade name (nome fantasia).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_name: Option<String>,
    /// Municipal registration number (inscrição municipal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipal_reg: Option<String>,
    /// IBGE municipality code of issuance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipality_code: Option<String>,
    /// Whether the company is enrolled in Simples Nacional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_national: Option<bool>,
    /// Special tax regime note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_regime: Option<String>,
    /// Postal address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CompanyAddressInput>,
}

/// Parameters for updating a company. Only provided fields change.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyUpdateParams {
    /// Registered legal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    /// Trade name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_name: Option<String>,
    /// Municipal registration number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipal_reg: Option<String>,
    /// IBGE municipality code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipality_code: Option<String>,
    /// Tax regime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_regime: Option<TaxRegime>,
    /// Whether the company is enrolled in Simples Nacional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_national: Option<bool>,
    /// Special tax regime note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_regime: Option<String>,
    /// Postal address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CompanyAddressInput>,
}

/// A company (invoice emitter) registered with Corvo.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    /// Unique id.
    pub id: String,
    /// Owning tenant id.
    pub tenant_id: String,
    /// Registered legal name.
    pub legal_name: String,
    /// Trade name, if set.
    pub trade_name: Option<String>,
    /// CNPJ, digits only.
    pub tax_id: String,
    /// Municipal registration number, if set.
    pub municipal_reg: Option<String>,
    /// IBGE municipality code, if set.
    pub municipality_code: Option<String>,
    /// Tax regime.
    pub tax_regime: TaxRegime,
    /// Simples Nacional enrollment.
    pub simple_national: bool,
    /// Special regime note, if set.
    pub special_regime: Option<String>,
    /// Address street, if provided.
    pub address_street: Option<String>,
    /// Address number, if provided.
    pub address_number: Option<String>,
    /// Address complement, if provided.
    pub address_complement: Option<String>,
    /// Address district, if provided.
    pub address_district: Option<String>,
    /// Address city.
    pub address_city: String,
    /// Address state code.
    pub address_state: String,
    /// Address CEP.
    pub address_zip: String,
    /// Fiscal schema version in use.
    pub fiscal_schema_version: String,
    /// Issuance environment (`test` or `live`).
    pub environment: String,
    /// Lifecycle status.
    pub status: String,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
    /// Last update timestamp (RFC 3339).
    pub updated_at: String,
    /// Deletion timestamp, if soft-deleted.
    pub deleted_at: Option<String>,
    /// Setup steps still missing before the company can issue.
    pub setup_gaps: Vec<String>,
}
