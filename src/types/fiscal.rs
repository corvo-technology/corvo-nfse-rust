//! Fiscal profile, service tax profile and DPS numbering types.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

wire_enum! {
    /// Tax regime of a fiscal profile.
    FiscalTaxRegime {
        /// MEI.
        Mei => "MEI",
        /// Simples Nacional.
        SimplesNacional => "SIMPLES_NACIONAL",
        /// Lucro Presumido.
        LucroPresumido => "LUCRO_PRESUMIDO",
        /// Lucro Real.
        LucroReal => "LUCRO_REAL",
        /// Another regime.
        OutroRegime => "OUTRO_REGIME",
        /// Unknown.
        Unknown => "UNKNOWN",
    }
}

wire_enum! {
    /// Review state of a profile.
    ReviewStatus {
        /// Not reviewed yet.
        Unreviewed => "unreviewed",
        /// Approved.
        Approved => "approved",
        /// Rejected.
        Rejected => "rejected",
    }
}

wire_enum! {
    /// How ISS is charged on a service.
    IssTaxation {
        /// Normal rate.
        Normal => "NORMAL",
        /// Fixed amount.
        Fixed => "FIXED",
        /// Exempt.
        Exempt => "EXEMPT",
    }
}

wire_enum! {
    /// Nature of the operation of a service.
    OperationNature {
        /// Service.
        Service => "SERVICE",
        /// Goods.
        Goods => "GOODS",
        /// Mixed.
        Mixed => "MIXED",
        /// Partner salon.
        PartnerSalon => "PARTNER_SALON",
    }
}

wire_enum! {
    /// What is known about withholdings of a service.
    WithholdingPolicy {
        /// Unknown.
        Unknown => "UNKNOWN",
        /// Confirmed none.
        NoneConfirmed => "NONE_CONFIRMED",
        /// Configured in `withholding_rates`.
        Configured => "CONFIGURED",
    }
}

wire_enum! {
    /// How a Simples Nacional company pays (`simplesApuracao`).
    SimplesApuracao {
        /// Simples Nacional.
        Sn => "SN",
        /// ISS due to the municipality.
        IssMunicipal => "ISS_MUNICIPAL",
        /// Regular.
        Regular => "REGULAR",
    }
}

/// A validity period of the fiscal profile of a company.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiscalProfile {
    /// Unique id.
    pub id: String,
    /// Owning tenant id.
    pub tenant_id: String,
    /// Company it belongs to.
    pub company_id: String,
    /// Tax regime.
    pub tax_regime: FiscalTaxRegime,
    /// Simples Nacional enrollment.
    pub simple_national: bool,
    /// How Simples Nacional is paid.
    #[serde(default)]
    pub simples_apuracao: Option<SimplesApuracao>,
    /// Effective ISS rate (%) of the Simples bracket.
    pub simples_iss_rate: Option<String>,
    /// Special tax regime (`0` to `6`).
    pub special_tax_regime: Option<String>,
    /// Whether the company opted into regular IBS/CBS.
    pub ibs_cbs_regular_option: bool,
    /// Day regular IBS/CBS starts (`YYYY-MM-DD`).
    pub ibs_cbs_regular_from: Option<String>,
    /// Where the profile came from.
    pub source: String,
    /// Review state.
    pub review_status: ReviewStatus,
    /// Who reviewed it.
    pub reviewed_by: Option<String>,
    /// When it was reviewed (RFC 3339).
    pub reviewed_at: Option<String>,
    /// First day it applies (`YYYY-MM-DD`).
    pub valid_from: String,
    /// Last day it applies (`YYYY-MM-DD`).
    pub valid_until: Option<String>,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
}

/// Parameters of [`FiscalProfiles::upsert`](crate::FiscalProfiles::upsert).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiscalProfileUpsertParams {
    /// Tax regime.
    pub tax_regime: FiscalTaxRegime,
    /// Send `true` to confirm a change that needs confirmation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// Day regular IBS/CBS starts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ibs_cbs_regular_from: Option<Option<String>>,
    /// Whether the company opted into regular IBS/CBS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ibs_cbs_regular_option: Option<bool>,
    /// Simples Nacional enrollment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simple_national: Option<bool>,
    /// `SN` (or `null`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simples_apuracao: Option<Option<SimplesApuracao>>,
    /// Special tax regime (`0` to `6`, or `null`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub special_tax_regime: Option<Option<String>>,
    /// First day it applies (`YYYY-MM-DD`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
}

impl FiscalProfileUpsertParams {
    /// Profile with `tax_regime` and no other field.
    pub fn new(tax_regime: FiscalTaxRegime) -> Self {
        Self {
            tax_regime,
            confirm: None,
            ibs_cbs_regular_from: None,
            ibs_cbs_regular_option: None,
            simple_national: None,
            simples_apuracao: None,
            special_tax_regime: None,
            valid_from: None,
        }
    }
}

/// Withholding rates (%) of a service, as decimal strings.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithholdingRates {
    /// COFINS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cofins: Option<String>,
    /// CSLL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csll: Option<String>,
    /// INSS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inss: Option<String>,
    /// IRRF.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub irrf: Option<String>,
    /// PIS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pis: Option<String>,
}

/// The tax classification of a service in the catalog of a company.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceTaxProfile {
    /// Unique id.
    pub id: String,
    /// Owning tenant id.
    pub tenant_id: String,
    /// Company it belongs to.
    pub company_id: String,
    /// Name.
    pub name: String,
    /// National service code (6 digits).
    pub service_code: String,
    /// Catalog version.
    pub catalog_version: Option<String>,
    /// IBS/CBS class code.
    pub class_code: Option<String>,
    /// Where the classification came from.
    pub classification_source: String,
    /// CST.
    pub cst: Option<String>,
    /// First day it applies (`YYYY-MM-DD`).
    pub effective_from: String,
    /// Last day it applies (`YYYY-MM-DD`).
    pub effective_until: Option<String>,
    /// IBGE code of the municipality of incidence.
    pub incidence_municipality_code: Option<String>,
    /// ISS rate (%).
    pub iss_rate: Option<String>,
    /// How ISS is charged.
    pub iss_taxation: Option<IssTaxation>,
    /// Whether ISS is withheld by default.
    pub iss_withholding_default: bool,
    /// Municipal service code.
    pub municipal_service_code: Option<String>,
    /// NBS code.
    pub nbs_code: Option<String>,
    /// Operation indicator.
    pub operation_indicator: Option<String>,
    /// Nature of the operation.
    pub operation_nature: OperationNature,
    /// Regimes it applies to.
    pub regime_applicability: Vec<FiscalTaxRegime>,
    /// Review state.
    pub review_status: ReviewStatus,
    /// What is known about withholdings.
    pub withholding_policy: WithholdingPolicy,
    /// Withholding rates.
    pub withholding_rates: WithholdingRates,
    /// Creation timestamp (RFC 3339).
    pub created_at: String,
}

/// Parameters for creating a service tax profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceTaxProfileCreateParams {
    /// Name (up to 120 characters).
    pub name: String,
    /// National service code (6 digits).
    pub service_code: String,
    /// First day it applies (`YYYY-MM-DD`).
    pub effective_from: String,
    /// Catalog version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<String>,
    /// IBS/CBS class code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class_code: Option<String>,
    /// Where the classification came from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification_source: Option<String>,
    /// CST.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cst: Option<String>,
    /// Last day it applies (`YYYY-MM-DD`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_until: Option<String>,
    /// IBGE code of the municipality of incidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incidence_municipality_code: Option<String>,
    /// ISS rate (%).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iss_rate: Option<String>,
    /// How ISS is charged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iss_taxation: Option<IssTaxation>,
    /// Whether ISS is withheld by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iss_withholding_default: Option<bool>,
    /// Municipal service code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub municipal_service_code: Option<String>,
    /// NBS code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbs_code: Option<String>,
    /// Operation indicator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_indicator: Option<String>,
    /// Nature of the operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_nature: Option<OperationNature>,
    /// Regimes it applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regime_applicability: Option<Vec<FiscalTaxRegime>>,
    /// What is known about withholdings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withholding_policy: Option<WithholdingPolicy>,
    /// Withholding rates, by tax name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withholding_rates: Option<BTreeMap<String, String>>,
}

impl ServiceTaxProfileCreateParams {
    /// Profile with the required fields.
    pub fn new(
        name: impl Into<String>,
        service_code: impl Into<String>,
        effective_from: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            service_code: service_code.into(),
            effective_from: effective_from.into(),
            catalog_version: None,
            class_code: None,
            classification_source: None,
            cst: None,
            effective_until: None,
            incidence_municipality_code: None,
            iss_rate: None,
            iss_taxation: None,
            iss_withholding_default: None,
            municipal_service_code: None,
            nbs_code: None,
            operation_indicator: None,
            operation_nature: None,
            regime_applicability: None,
            withholding_policy: None,
            withholding_rates: None,
        }
    }
}

/// Change to a service tax profile. Only provided fields change.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceTaxProfileUpdateParams {
    /// Name (up to 120 characters).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// National service code (6 digits).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// First day it applies (`YYYY-MM-DD`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_from: Option<String>,
    /// Catalog version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<String>,
    /// IBS/CBS class code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class_code: Option<String>,
    /// Where the classification came from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification_source: Option<String>,
    /// CST.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cst: Option<String>,
    /// Last day it applies (`YYYY-MM-DD`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_until: Option<String>,
    /// IBGE code of the municipality of incidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incidence_municipality_code: Option<String>,
    /// ISS rate (%).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iss_rate: Option<String>,
    /// How ISS is charged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iss_taxation: Option<IssTaxation>,
    /// Whether ISS is withheld by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iss_withholding_default: Option<bool>,
    /// Municipal service code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub municipal_service_code: Option<String>,
    /// NBS code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbs_code: Option<String>,
    /// Operation indicator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_indicator: Option<String>,
    /// Nature of the operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_nature: Option<OperationNature>,
    /// Regimes it applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regime_applicability: Option<Vec<FiscalTaxRegime>>,
    /// What is known about withholdings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withholding_policy: Option<WithholdingPolicy>,
    /// Withholding rates, by tax name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withholding_rates: Option<BTreeMap<String, String>>,
}

/// The DPS numbering of a company.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DpsSequence {
    /// Last number used.
    pub last_number: i64,
    /// DPS series.
    pub series: String,
}

/// Body of [`DpsSequenceResource::set`](crate::DpsSequenceResource::set).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DpsSequenceSetParams {
    /// The last number issued elsewhere; the next invoice uses the following
    /// one. Below the current number fails with `DPS_NUMBER_BELOW_CURRENT`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_number: Option<i64>,
}
