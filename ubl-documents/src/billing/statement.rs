// UBL Statement — billing document type.
// UBL element: maindoc:Statement

use serde::{Deserialize, Serialize};
use ubl_common::cbc::*;
use ubl_common::cac::allowance::AllowanceCharge;
use ubl_common::cac::customer::CustomerParty;
use ubl_common::cac::document::DocumentReference;
use ubl_common::cac::document::Signature;
use ubl_common::cac::party::Party;
use ubl_common::cac::payment::{PaymentMeans, PaymentTerms};
use ubl_common::cac::period::Period;
use ubl_common::cac::statement_line::StatementLine;
use ubl_common::cac::supplier::SupplierParty;
use ubl_common::cac::tax::TaxTotal;

/// A periodic account statement showing transactions and balances.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statement {
    pub id: ID,
    pub copy_indicator: Option<CopyIndicator>,
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    pub issue_time: Option<IssueTime>,
    pub note: Vec<Note>,
    pub document_currency_code: DocumentCurrencyCode,
    pub total_debit_amount: Option<TotalDebitAmount>,
    pub total_credit_amount: Option<TotalCreditAmount>,
    pub total_balance_amount: Option<TotalBalanceAmount>,
    pub line_count_numeric: Option<LineCountNumeric>,
    pub statement_type_code: Option<StatementTypeCode>,
    pub statement_period: Vec<Period>,
    pub additional_document_reference: Vec<DocumentReference>,
    pub signature: Vec<Signature>,
    pub accounting_supplier_party: SupplierParty,
    pub accounting_customer_party: CustomerParty,
    pub buyer_customer_party: Option<CustomerParty>,
    pub seller_supplier_party: Option<SupplierParty>,
    pub originator_customer_party: Option<CustomerParty>,
    pub beneficiary_party: Option<Party>,
    pub payee_party: Option<Party>,
    pub payment_means: Vec<PaymentMeans>,
    pub payment_terms: Vec<PaymentTerms>,
    pub allowance_charge: Vec<AllowanceCharge>,
    pub tax_total: Vec<TaxTotal>,
    pub statement_line: Vec<StatementLine>,
}
