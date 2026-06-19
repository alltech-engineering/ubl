#[derive(Debug, Deserialize, Serialize)]
/// A class to define a monetary total.
///
/// UBL Dictionary Entry Name: `Monetary Total. Details`
///
/// Generated from XSD type `MonetaryTotalType`.
pub struct MonetaryTotal {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The monetary amount of an extended transaction line, net of tax and settlement discounts, but
/// inclusive of any applicable rounding amount.
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
/// The monetary amount of an extended transaction line, exclusive of taxes.
    #[serde(default, rename = "TaxExclusiveAmount")]
    pub tax_exclusive_amount: Option<cct::Amount>,
/// The monetary amount including taxes; the sum of payable amount and prepaid amount.
    #[serde(default, rename = "TaxInclusiveAmount")]
    pub tax_inclusive_amount: Option<cct::Amount>,
/// The total monetary amount of all allowances.
    #[serde(default, rename = "AllowanceTotalAmount")]
    pub allowance_total_amount: Option<cct::Amount>,
/// The total monetary amount of all allowances, inclusive of all taxes.
    #[serde(default, rename = "AllowanceTotalTaxInclusiveAmount")]
    pub allowance_total_tax_inclusive_amount: Option<cct::Amount>,
/// The total monetary amount of all charges.
    #[serde(default, rename = "ChargeTotalAmount")]
    pub charge_total_amount: Option<cct::Amount>,
/// The total monetary amount of all charges, inclusive of all taxes.
    #[serde(default, rename = "ChargeTotalTaxInclusiveAmount")]
    pub charge_total_tax_inclusive_amount: Option<cct::Amount>,
/// The total withholding tax amount.
    #[serde(default, rename = "WithholdingTaxTotalAmount")]
    pub withholding_tax_total_amount: Option<cct::Amount>,
/// The total prepaid monetary amount.
    #[serde(default, rename = "PrepaidAmount")]
    pub prepaid_amount: Option<cct::Amount>,
/// The rounding amount (positive or negative) added to produce the line extension amount.
    #[serde(default, rename = "PayableRoundingAmount")]
    pub payable_rounding_amount: Option<cct::Amount>,
/// The amount of the monetary total to be paid.
    #[serde(rename = "PayableAmount")]
    pub payable_amount: cct::Amount,
/// The amount of the monetary total to be paid, expressed in an alternative currency.
    #[serde(default, rename = "PayableAlternativeAmount")]
    pub payable_alternative_amount: Option<cct::Amount>,
}
