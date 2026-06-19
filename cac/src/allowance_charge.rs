#[derive(Debug, Deserialize, Serialize)]
pub struct AllowanceCharge {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(rename = "ChargeIndicator")]
    pub charge_indicator: udt::Indicator,
    #[serde(default, rename = "AllowanceChargeReasonCode")]
    pub allowance_charge_reason_code: Option<cct::Code>,
    #[serde(default, rename = "AllowanceChargeReason")]
    pub allowance_charge_reason: Vec<cct::Text>,
    #[serde(default, rename = "MultiplierFactorNumeric")]
    pub multiplier_factor_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "PrepaidIndicator")]
    pub prepaid_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "SequenceNumeric")]
    pub sequence_numeric: Option<cct::Numeric>,
    #[serde(rename = "Amount")]
    pub amount: cct::Amount,
    #[serde(default, rename = "TaxInclusiveAmount")]
    pub tax_inclusive_amount: Option<cct::Amount>,
    #[serde(default, rename = "BaseAmount")]
    pub base_amount: Option<cct::Amount>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
    #[serde(default, rename = "PerUnitAmount")]
    pub per_unit_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxCategory")]
    pub tax_category: Vec<TaxCategory>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Option<TaxTotal>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Vec<PaymentMeans>,
}
