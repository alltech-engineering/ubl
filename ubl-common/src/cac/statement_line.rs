// UBL Statement line — a line on an account statement document.
// UBL element: cac:StatementLine

use serde::{Deserialize, Serialize};
use crate::cbc::*;

/// A class to define a line in a Statement document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatementLine {
    pub id: ID,
    pub uuid: Option<UUID>,
    #[serde(default)]
    pub note: Vec<Note>,
    pub balance_brought_forward_indicator: Option<Indicator>,
    pub debit_line_amount: Option<DebitLineAmount>,
    pub credit_line_amount: Option<CreditLineAmount>,
    pub balance_amount: Option<BalanceAmount>,
    pub payment_purpose_code: Option<PaymentPurposeCode>,
    #[serde(default)]
    pub payment_means: Vec<PaymentMeans>,
    #[serde(default)]
    pub payment_terms: Vec<PaymentTerms>,
    pub buyer_customer_party: Option<CustomerParty>,
    pub seller_supplier_party: Option<SupplierParty>,
    pub originator_customer_party: Option<CustomerParty>,
    pub accounting_customer_party: Option<CustomerParty>,
    pub accounting_supplier_party: Option<SupplierParty>,
    pub payee_party: Option<Party>,
    #[serde(default)]
    pub invoice_period: Vec<Period>,
    #[serde(default)]
    pub billing_reference: Vec<BillingReference>,
    #[serde(default)]
    pub document_reference: Vec<DocumentReference>,
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
use crate::cac::payment::{PaymentMeans, PaymentTerms};
use crate::cac::period::Period;
use crate::cac::supplier::SupplierParty;
