#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in a Reminder document.
///
/// UBL Dictionary Entry Name: `Reminder Line. Details`
///
/// Generated from XSD type `ReminderLineType`.
pub struct ReminderLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this reminder line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A universally unique identifier for this reminder line.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// An indication that this reminder line contains a balance brought forward (true) or does not (false).
    #[serde(default, rename = "BalanceBroughtForwardIndicator")]
    pub balance_brought_forward_indicator: Option<udt::Indicator>,
/// The amount debited on this reminder line.
    #[serde(default, rename = "DebitLineAmount")]
    pub debit_line_amount: Option<cct::Amount>,
/// The amount credited on this reminder line.
    #[serde(default, rename = "CreditLineAmount")]
    pub credit_line_amount: Option<cct::Amount>,
/// The buyer's accounting cost centre for this reminder line, expressed as a code.
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
/// The buyer's accounting cost centre for this reminder line, expressed as text.
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
/// The penalty for late payment, expressed as a percentage.
    #[serde(default, rename = "PenaltySurchargePercent")]
    pub penalty_surcharge_percent: Option<cct::Numeric>,
/// The amount on this reminder line.
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
/// A code signifying the business purpose for this payment.
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: Option<cct::Code>,
/// A period to which this reminder line applies.
    #[serde(default, rename = "ReminderPeriod")]
    pub reminder_period: Vec<Period>,
/// A reference to a billing document associated with this reminder line.
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<BillingReference>,
/// The rate of exchange between the currency of the Reminder and the currency of the document described
/// in the BillingReference.
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Option<ExchangeRate>,
}
