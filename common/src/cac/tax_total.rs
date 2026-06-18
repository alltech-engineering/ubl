#[derive(Debug, Deserialize, Serialize)]
pub struct TaxTotal {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "TaxAmount")]
    pub tax_amount: super::cct::AmountType,
    #[serde(default, rename = "CalculationSequenceNumeric")]
    pub calculation_sequence_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "RoundingAmount")]
    pub rounding_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxEvidenceIndicator")]
    pub tax_evidence_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "TaxIncludedIndicator")]
    pub tax_included_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "TaxSubtotal")]
    pub tax_subtotal: Vec<TaxSubtotal>,
}
