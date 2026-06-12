// UBL RemittanceAdvice — billing document type.
// UBL element: maindoc:RemittanceAdvice

use serde::{Deserialize, Serialize};
use ubl_common::cbc::*;
use ubl_common::cac::customer::CustomerParty;
use ubl_common::cac::document::DocumentReference;
use ubl_common::cac::document::Signature;
use ubl_common::cac::order_reference::BillingReference;
use ubl_common::cac::party::Party;
use ubl_common::cac::payment::PaymentMeans;
use ubl_common::cac::period::Period;
use ubl_common::cac::remittance_advice_line::RemittanceAdviceLine;
use ubl_common::cac::supplier::SupplierParty;
use ubl_common::cac::tax::TaxTotal;

/// Notification of payment made — advises the supplier that payment has been sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemittanceAdvice {
    pub id: ID,
    pub copy_indicator: Option<CopyIndicator>,
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    pub issue_time: Option<IssueTime>,
    pub note: Vec<Note>,
    pub document_currency_code: DocumentCurrencyCode,
    pub total_debit_amount: Option<TotalDebitAmount>,
    pub total_credit_amount: Option<TotalCreditAmount>,
    pub total_payment_amount: Option<TotalPaymentAmount>,
    pub payment_order_reference: Option<PaymentOrderReference>,
    pub payer_reference: Option<PayerReference>,
    pub invoicing_party_reference: Option<InvoicingPartyReference>,
    pub line_count_numeric: Option<LineCountNumeric>,
    pub invoice_period: Vec<Period>,
    pub billing_reference: Vec<BillingReference>,
    pub additional_document_reference: Vec<DocumentReference>,
    pub signature: Vec<Signature>,
    pub accounting_customer_party: CustomerParty,
    pub accounting_supplier_party: SupplierParty,
    pub payee_party: Option<Party>,
    pub payment_means: Vec<PaymentMeans>,
    pub tax_total: Vec<TaxTotal>,
    pub remittance_advice_line: Vec<RemittanceAdviceLine>,
}
