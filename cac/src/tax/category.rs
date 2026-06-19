#[derive(Debug, Deserialize, Serialize)]
/// A class to describe one of the tax categories within a taxation scheme (e.g., High Rate VAT, Low
/// Rate VAT).
///
/// UBL Dictionary Entry Name: `Tax Category. Details`
///
/// Generated from XSD type `TaxCategoryType`.
pub struct TaxCategory {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this tax category.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The name of this tax category.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// The tax rate for this category, expressed as a percentage.
    #[serde(default, rename = "Percent")]
    pub percent: Option<cct::Numeric>,
/// A Unit of Measures used as the basic for the tax calculation applied at a certain rate per unit.
    #[serde(default, rename = "BaseUnitMeasure")]
    pub base_unit_measure: Option<cct::Measure>,
/// Where a tax is applied at a certain rate per unit, the rate per unit applied.
    #[serde(default, rename = "PerUnitAmount")]
    pub per_unit_amount: Option<cct::Amount>,
/// The reason for tax being exempted, expressed as a code.
    #[serde(default, rename = "TaxExemptionReasonCode")]
    pub tax_exemption_reason_code: Option<cct::Code>,
/// The reason for tax being exempted, expressed as text.
    #[serde(default, rename = "TaxExemptionReason")]
    pub tax_exemption_reason: Vec<cct::Text>,
/// Where a tax is tiered, the range of taxable amounts that determines the rate of tax applicable to
/// this tax category.
    #[serde(default, rename = "TierRange")]
    pub tier_range: Option<cct::Text>,
/// Where a tax is tiered, the tax rate that applies within the specified range of taxable amounts for
/// this tax category.
    #[serde(default, rename = "TierRatePercent")]
    pub tier_rate_percent: Option<cct::Numeric>,
/// A code signifying the type of supply to which this tax category applies, such as goods, services, or
/// a mixture.
    #[serde(default, rename = "SupplyTypeCode")]
    pub supply_type_code: Option<cct::Code>,
/// The taxation scheme within which this tax category is defined.
    #[serde(default, rename = "TaxScheme")]
    pub tax_scheme: Option<TaxScheme>,
}
