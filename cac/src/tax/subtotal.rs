#[derive(Debug, Deserialize, Serialize)]
pub struct TaxSubtotal {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "TaxableAmount")]
    pub taxable_amount: Option<cct::Amount>,
    #[serde(rename = "TaxAmount")]
    pub tax_amount: cct::Amount,
    #[serde(default, rename = "TaxInclusiveAmount")]
    pub tax_inclusive_amount: Option<cct::Amount>,
    #[serde(default, rename = "CalculationSequenceNumeric")]
    pub calculation_sequence_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "TransactionCurrencyTaxAmount")]
    pub transaction_currency_tax_amount: Option<cct::Amount>,
    #[serde(default, rename = "Percent")]
    pub percent: Option<cct::Numeric>,
    #[serde(default, rename = "BaseUnitMeasure")]
    pub base_unit_measure: Option<cct::Measure>,
    #[serde(default, rename = "PerUnitAmount")]
    pub per_unit_amount: Option<cct::Amount>,
    #[serde(default, rename = "TierRange")]
    pub tier_range: Option<cct::Text>,
    #[serde(default, rename = "TierRatePercent")]
    pub tier_rate_percent: Option<cct::Numeric>,
    #[serde(rename = "TaxCategory")]
    pub tax_category: TaxCategory,
    #[serde(default, rename = "TaxDueCountry")]
    pub tax_due_country: Option<crate::Country>,
}
