#[derive(Debug, Deserialize, Serialize)]
pub struct ReminderLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "BalanceBroughtForwardIndicator")]
    pub balance_brought_forward_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "DebitLineAmount")]
    pub debit_line_amount: Option<cct::Amount>,
    #[serde(default, rename = "CreditLineAmount")]
    pub credit_line_amount: Option<cct::Amount>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
    #[serde(default, rename = "PenaltySurchargePercent")]
    pub penalty_surcharge_percent: Option<cct::Numeric>,
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: Option<cct::Code>,
    #[serde(default, rename = "ReminderPeriod")]
    pub reminder_period: Vec<Period>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<BillingReference>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Option<ExchangeRate>,
}
