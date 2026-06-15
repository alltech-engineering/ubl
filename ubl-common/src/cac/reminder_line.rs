// UBL Reminder line — a line on a payment reminder document.
// UBL element: cac:ReminderLine

use crate::cbc::*;
use serde::{Deserialize, Serialize};

/// A class to define a line in a Reminder document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReminderLine {
    pub id: ID,
    #[serde(default)]
    pub uuid: Option<UUID>,
    #[serde(default)]
    pub note: Vec<Note>,
    #[serde(default)]
    pub balance_brought_forward_indicator: Option<Indicator>,
    #[serde(default)]
    pub debit_line_amount: Option<DebitLineAmount>,
    #[serde(default)]
    pub credit_line_amount: Option<CreditLineAmount>,
    #[serde(default)]
    pub accounting_cost_code: Option<AccountingCostCode>,
    #[serde(default)]
    pub accounting_cost: Option<AccountingCost>,
    #[serde(default)]
    pub penalty_surcharge_percent: Option<Percent>,
    #[serde(default)]
    pub payment_purpose_code: Option<PaymentPurposeCode>,
    #[serde(default)]
    pub reminder_period: Vec<Period>,
    #[serde(default)]
    pub billing_reference: Vec<BillingReference>,
    #[serde(default)]
    pub exchange_rate: Option<ExchangeRate>,
    #[serde(default)]
    pub document_reference: Vec<DocumentReference>,
}

use crate::cac::document::DocumentReference;
use crate::cac::exchange_rate::ExchangeRate;
use crate::cac::order_reference::BillingReference;
use crate::cac::period::Period;
