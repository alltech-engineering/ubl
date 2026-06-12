// UBL Reminder line — a line on a payment reminder document.
// UBL element: cac:ReminderLine

use serde::{Deserialize, Serialize};
use crate::cbc::*;

/// A class to define a line in a Reminder document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReminderLine {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub note: Vec<Note>,
    pub balance_brought_forward_indicator: Option<Indicator>,
    pub debit_line_amount: Option<DebitLineAmount>,
    pub credit_line_amount: Option<CreditLineAmount>,
    pub accounting_cost_code: Option<AccountingCostCode>,
    pub accounting_cost: Option<AccountingCost>,
    pub penalty_surcharge_percent: Option<Percent>,
    pub payment_purpose_code: Option<PaymentPurposeCode>,
    pub reminder_period: Vec<Period>,
    pub billing_reference: Vec<BillingReference>,
    pub exchange_rate: Option<ExchangeRate>,
    pub document_reference: Vec<DocumentReference>,
}

use crate::cac::document::DocumentReference;
use crate::cac::exchange_rate::ExchangeRate;
use crate::cac::order_reference::BillingReference;
use crate::cac::period::Period;
