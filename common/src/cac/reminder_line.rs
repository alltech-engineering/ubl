#[derive(Debug, Deserialize, Serialize)]
pub struct ReminderLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "BalanceBroughtForwardIndicator")]
    pub balance_brought_forward_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "DebitLineAmount")]
    pub debit_line_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "CreditLineAmount")]
    pub credit_line_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<super::cct::TextType>,
    #[serde(default, rename = "PenaltySurchargePercent")]
    pub penalty_surcharge_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "Amount")]
    pub amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ReminderPeriod")]
    pub reminder_period: Vec<Period>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<BillingReference>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Option<ExchangeRate>,
}
