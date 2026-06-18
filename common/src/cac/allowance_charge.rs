#[derive(Debug, Deserialize, Serialize)]
pub struct AllowanceCharge {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(rename = "ChargeIndicator")]
    pub charge_indicator: super::udt::IndicatorType,
    #[serde(default, rename = "AllowanceChargeReasonCode")]
    pub allowance_charge_reason_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "AllowanceChargeReason")]
    pub allowance_charge_reason: Vec<super::cct::TextType>,
    #[serde(default, rename = "MultiplierFactorNumeric")]
    pub multiplier_factor_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "PrepaidIndicator")]
    pub prepaid_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "SequenceNumeric")]
    pub sequence_numeric: Option<super::cct::NumericType>,
    #[serde(rename = "Amount")]
    pub amount: super::cct::AmountType,
    #[serde(default, rename = "TaxInclusiveAmount")]
    pub tax_inclusive_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "BaseAmount")]
    pub base_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<super::cct::TextType>,
    #[serde(default, rename = "PerUnitAmount")]
    pub per_unit_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxCategory")]
    pub tax_category: Vec<TaxCategory>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Option<TaxTotal>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Vec<PaymentMeans>,
}
