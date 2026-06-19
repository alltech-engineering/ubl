#[derive(Debug, Deserialize, Serialize)]
pub struct MonetaryTotal {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxExclusiveAmount")]
    pub tax_exclusive_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxInclusiveAmount")]
    pub tax_inclusive_amount: Option<cct::Amount>,
    #[serde(default, rename = "AllowanceTotalAmount")]
    pub allowance_total_amount: Option<cct::Amount>,
    #[serde(default, rename = "AllowanceTotalTaxInclusiveAmount")]
    pub allowance_total_tax_inclusive_amount: Option<cct::Amount>,
    #[serde(default, rename = "ChargeTotalAmount")]
    pub charge_total_amount: Option<cct::Amount>,
    #[serde(default, rename = "ChargeTotalTaxInclusiveAmount")]
    pub charge_total_tax_inclusive_amount: Option<cct::Amount>,
    #[serde(default, rename = "WithholdingTaxTotalAmount")]
    pub withholding_tax_total_amount: Option<cct::Amount>,
    #[serde(default, rename = "PrepaidAmount")]
    pub prepaid_amount: Option<cct::Amount>,
    #[serde(default, rename = "PayableRoundingAmount")]
    pub payable_rounding_amount: Option<cct::Amount>,
    #[serde(rename = "PayableAmount")]
    pub payable_amount: cct::Amount,
    #[serde(default, rename = "PayableAlternativeAmount")]
    pub payable_alternative_amount: Option<cct::Amount>,
}
