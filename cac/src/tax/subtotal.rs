#[derive(Debug, Deserialize, Serialize)]
/// A class to define the subtotal for a particular tax category within a particular taxation scheme,
/// such as standard rate within VAT.
///
/// UBL Dictionary Entry Name: `Tax Subtotal. Details`
///
/// Generated from XSD type `TaxSubtotalType`.
pub struct TaxSubtotal {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// The net amount to which the tax percent (rate) is applied to calculate the tax amount.
    #[serde(default, rename = "TaxableAmount")]
    pub taxable_amount: Option<cct::Amount>,
/// The amount of this tax subtotal.
    #[serde(rename = "TaxAmount")]
    pub tax_amount: cct::Amount,
/// The total amount after the tax amount has been added to the taxable amount.
    #[serde(default, rename = "TaxInclusiveAmount")]
    pub tax_inclusive_amount: Option<cct::Amount>,
/// The number of this tax subtotal in the sequence of subtotals corresponding to the order in which
/// multiple taxes are applied. If all taxes are applied to the same taxable amount (i.e., their order
/// of application is inconsequential), then CalculationSequenceNumeric is 1 for all tax subtotals
/// applied to a given amount.
    #[serde(default, rename = "CalculationSequenceNumeric")]
    pub calculation_sequence_numeric: Option<cct::Numeric>,
/// The amount of this tax subtotal, expressed in the currency used for invoicing.
    #[serde(default, rename = "TransactionCurrencyTaxAmount")]
    pub transaction_currency_tax_amount: Option<cct::Amount>,
/// The tax rate of the tax category applied to this tax subtotal, expressed as a percentage.
    #[serde(default, rename = "Percent")]
    pub percent: Option<cct::Numeric>,
/// The unit of measure on which the tax calculation is based
    #[serde(default, rename = "BaseUnitMeasure")]
    pub base_unit_measure: Option<cct::Measure>,
/// Where a tax is applied at a certain rate per unit, the rate per unit applied.
    #[serde(default, rename = "PerUnitAmount")]
    pub per_unit_amount: Option<cct::Amount>,
/// Where a tax is tiered, the range of taxable amounts that determines the rate of tax applicable to
/// this tax subtotal.
    #[serde(default, rename = "TierRange")]
    pub tier_range: Option<cct::Text>,
/// Where a tax is tiered, the tax rate that applies within a specified range of taxable amounts for
/// this tax subtotal.
    #[serde(default, rename = "TierRatePercent")]
    pub tier_rate_percent: Option<cct::Numeric>,
/// The tax category applicable to this subtotal.
    #[serde(rename = "TaxCategory")]
    pub tax_category: TaxCategory,
/// The country where this tax is due.
    #[serde(default, rename = "TaxDueCountry")]
    pub tax_due_country: Option<crate::Country>,
}
