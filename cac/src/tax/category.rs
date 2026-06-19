#[derive(Debug, Deserialize, Serialize)]
pub struct TaxCategory {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "Percent")]
    pub percent: Option<cct::Numeric>,
    #[serde(default, rename = "BaseUnitMeasure")]
    pub base_unit_measure: Option<cct::Measure>,
    #[serde(default, rename = "PerUnitAmount")]
    pub per_unit_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxExemptionReasonCode")]
    pub tax_exemption_reason_code: Option<cct::Code>,
    #[serde(default, rename = "TaxExemptionReason")]
    pub tax_exemption_reason: Vec<cct::Text>,
    #[serde(default, rename = "TierRange")]
    pub tier_range: Option<cct::Text>,
    #[serde(default, rename = "TierRatePercent")]
    pub tier_rate_percent: Option<cct::Numeric>,
    #[serde(default, rename = "SupplyTypeCode")]
    pub supply_type_code: Option<cct::Code>,
    #[serde(default, rename = "TaxScheme")]
    pub tax_scheme: Option<TaxScheme>,
}
