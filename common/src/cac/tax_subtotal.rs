#[derive(Debug, Deserialize, Serialize)]
pub struct TaxSubtotal {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "TaxableAmount")]
    pub taxable_amount: Option<super::cct::AmountType>,
    #[serde(rename = "TaxAmount")]
    pub tax_amount: super::cct::AmountType,
    #[serde(default, rename = "TaxInclusiveAmount")]
    pub tax_inclusive_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "CalculationSequenceNumeric")]
    pub calculation_sequence_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "TransactionCurrencyTaxAmount")]
    pub transaction_currency_tax_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "Percent")]
    pub percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "BaseUnitMeasure")]
    pub base_unit_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "PerUnitAmount")]
    pub per_unit_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TierRange")]
    pub tier_range: Option<super::cct::TextType>,
    #[serde(default, rename = "TierRatePercent")]
    pub tier_rate_percent: Option<super::cct::NumericType>,
    #[serde(rename = "TaxCategory")]
    pub tax_category: TaxCategory,
    #[serde(default, rename = "TaxDueCountry")]
    pub tax_due_country: Option<Country>,
}
