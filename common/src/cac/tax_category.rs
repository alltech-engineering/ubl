#[derive(Debug, Deserialize, Serialize)]
pub struct TaxCategory {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "Percent")]
    pub percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "BaseUnitMeasure")]
    pub base_unit_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "PerUnitAmount")]
    pub per_unit_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxExemptionReasonCode")]
    pub tax_exemption_reason_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "TaxExemptionReason")]
    pub tax_exemption_reason: Vec<super::cct::TextType>,
    #[serde(default, rename = "TierRange")]
    pub tier_range: Option<super::cct::TextType>,
    #[serde(default, rename = "TierRatePercent")]
    pub tier_rate_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "SupplyTypeCode")]
    pub supply_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "TaxScheme")]
    pub tax_scheme: Option<TaxScheme>,
}
