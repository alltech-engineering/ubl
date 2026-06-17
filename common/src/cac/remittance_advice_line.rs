// UBL RemittanceAdvice line — a line on a remittance advice document.
// UBL element: cac:RemittanceAdviceLine

use crate::cbc::*;
use serde::{Deserialize, Serialize};

/// A class to define a line in a RemittanceAdvice document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemittanceAdviceLine {
    pub id: ID,
    #[serde(default)]
    pub uuid: Option<UUID>,
    #[serde(default)]
    pub note: Vec<Note>,
    #[serde(default)]
    pub debit_line_amount: Option<DebitLineAmount>,
    #[serde(default)]
    pub credit_line_amount: Option<CreditLineAmount>,
    #[serde(default)]
    pub balance_amount: Option<BalanceAmount>,
    #[serde(default)]
    pub payment_purpose_code: Option<PaymentPurposeCode>,
    #[serde(default)]
    pub accounting_supplier_party: Option<SupplierParty>,
    #[serde(default)]
    pub accounting_customer_party: Option<CustomerParty>,
    #[serde(default)]
    pub buyer_customer_party: Option<CustomerParty>,
    #[serde(default)]
    pub seller_supplier_party: Option<SupplierParty>,
    #[serde(default)]
    pub originator_customer_party: Option<CustomerParty>,
    #[serde(default)]
    pub payee_party: Option<Party>,
    #[serde(default)]
    pub invoice_period: Vec<Period>,
    #[serde(default)]
    pub billing_reference: Vec<BillingReference>,
    #[serde(default)]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default)]
    pub exchange_rate: Option<ExchangeRate>,
    #[serde(default)]
    pub allowance_charge: Vec<AllowanceCharge>,
}

use crate::cac::allowance::AllowanceCharge;
use crate::cac::customer::CustomerParty;
use crate::cac::document::DocumentReference;
use crate::cac::exchange_rate::ExchangeRate;
use crate::cac::order_reference::BillingReference;
use crate::cac::party::Party;
use crate::cac::period::Period;
use crate::cac::supplier::SupplierParty;
