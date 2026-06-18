#[derive(Debug, Deserialize, Serialize)]
pub struct MonetaryTotal {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxExclusiveAmount")]
    pub tax_exclusive_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveAmount")]
    pub tax_inclusive_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "AllowanceTotalAmount")]
    pub allowance_total_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "AllowanceTotalTaxInclusiveAmount")]
    pub allowance_total_tax_inclusive_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "ChargeTotalAmount")]
    pub charge_total_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "ChargeTotalTaxInclusiveAmount")]
    pub charge_total_tax_inclusive_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "WithholdingTaxTotalAmount")]
    pub withholding_tax_total_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "PrepaidAmount")]
    pub prepaid_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "PayableRoundingAmount")]
    pub payable_rounding_amount: Option<super::cct::AmountType>,
    #[serde(rename = "PayableAmount")]
    pub payable_amount: super::cct::AmountType,
    #[serde(default, rename = "PayableAlternativeAmount")]
    pub payable_alternative_amount: Option<super::cct::AmountType>,
}
