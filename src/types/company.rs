//! Company (emitter) types.

use serde::{Deserialize, Serialize};

/// Brazilian tax regime of a company. Corvo serves only these two regimes; any
/// other value is rejected with `422` `ERROR_TAX_REGIME_NOT_SUPPORTED`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxRegime {
    /// Microempreendedor Individual.
    #[serde(rename = "MEI")]
    Mei,
    /// Simples Nacional.
    #[serde(rename = "Simples Nacional")]
    SimplesNacional,
}

impl TaxRegime {
    /// The exact string the API expects on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            TaxRegime::Mei => "MEI",
            TaxRegime::SimplesNacional => "Simples Nacional",
        }
    }
}

/// Postal address input for a company. Every field is optional.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyAddressInput {
    /// Street name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// Street number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// Complement (suite, floor, …).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complement: Option<String>,
    /// District (bairro).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    /// City name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter state code (UF).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// CEP, 8 digits with or without punctuation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

/// Parameters for creating a company.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyCreateParams {
    /// Registered legal name (razão social).
    pub legal_name: String,
    /// CNPJ.
    pub tax_id: String,
    /// Tax regime.
    pub tax_regime: TaxRegime,
    /// IBGE code of the company's city (7 digits).
    pub municipality_code: String,
    /// Trade name (nome fantasia).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub trade_name: Option<String>,
    /// Municipal registration. Stored, but only sent on the invoice when
    /// `send_municipal_reg` is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipal_reg: Option<String>,
    /// Default `false`: turn it on only if the city registered the company in
    /// the national registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_municipal_reg: Option<bool>,
    /// National service code (6 digits) used when an invoice omits `service.code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_service_code: Option<String>,
    /// Whether the company is enrolled in Simples Nacional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_national: Option<bool>,
    /// Special tax regime note.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub special_regime: Option<String>,
    /// Required for `Simples Nacional`, ignored for `MEI`. Effective total rate
    /// (%) of your DAS, as in the PGDAS-D, e.g. `"6.54"` (0 to 100, at most 2
    /// decimals).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub simples_rate: Option<String>,
    /// `Simples Nacional` only: effective ISS rate (%) of your bracket (1.80 to
    /// 5.00), needed when a customer withholds ISS.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub simples_iss_rate: Option<String>,
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
    /// Whether to send the municipal registration on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_municipal_reg: Option<bool>,
    /// IBGE municipality code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipality_code: Option<String>,
    /// National service code (6 digits).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_service_code: Option<String>,
    /// Tax regime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_regime: Option<TaxRegime>,
    /// Whether the company is enrolled in Simples Nacional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_national: Option<bool>,
    /// Special tax regime note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_regime: Option<String>,
    /// New Simples rate (%), e.g. `"6.54"`. Omit to keep the current value.
    /// Ignored for `MEI`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simples_rate: Option<String>,
    /// New ISS rate (%) of the Simples bracket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simples_iss_rate: Option<String>,
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
    #[serde(default)]
    pub municipal_reg: Option<String>,
    /// Whether the municipal registration is sent on the invoice.
    #[serde(default)]
    pub send_municipal_reg: Option<bool>,
    /// IBGE municipality code, if set.
    #[serde(default)]
    pub municipality_code: Option<String>,
    /// Default national service code, if set.
    #[serde(default)]
    pub default_service_code: Option<String>,
    /// Tax regime.
    #[serde(default)]
    pub tax_regime: Option<TaxRegime>,
    /// Simples Nacional enrollment.
    pub simple_national: bool,
    /// Special regime note, if set.
    pub special_regime: Option<String>,
    /// Address street, if provided.
    #[serde(default)]
    pub address_street: Option<String>,
    /// Address number, if provided.
    #[serde(default)]
    pub address_number: Option<String>,
    /// Address complement, if provided.
    #[serde(default)]
    pub address_complement: Option<String>,
    /// Address district, if provided.
    #[serde(default)]
    pub address_district: Option<String>,
    /// Address city, if provided.
    #[serde(default)]
    pub address_city: Option<String>,
    /// Address state code, if provided.
    #[serde(default)]
    pub address_state: Option<String>,
    /// Address CEP, if provided.
    #[serde(default)]
    pub address_zip: Option<String>,
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
    #[serde(default)]
    pub deleted_at: Option<String>,
    /// Setup steps still missing before the company can issue
    /// (`municipality_code`, `certificate`).
    pub setup_gaps: Vec<String>,
}
