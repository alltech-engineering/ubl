// UBL Reminder — billing document type.
// UBL element: maindoc:Reminder

use serde::{Deserialize, Serialize};
use ubl_common::cbc::*;
use ubl_common::cac::allowance::AllowanceCharge;
use ubl_common::cac::customer::CustomerParty;
use ubl_common::cac::document::DocumentReference;
use ubl_common::cac::document::Signature;
use ubl_common::cac::exchange_rate::ExchangeRate;
use ubl_common::cac::party::Party;
use ubl_common::cac::payment::{PaymentMeans, PaymentTerms};
use ubl_common::cac::period::Period;
use ubl_common::cac::prepaid_payment::PrepaidPayment;
use ubl_common::cac::reminder_line::ReminderLine;
use ubl_common::cac::supplier::SupplierParty;
use ubl_common::cac::tax::TaxTotal;
use ubl_common::cac::totals::LegalTotal;

/// A payment reminder for overdue invoices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reminder {
    pub id: ID,
    pub copy_indicator: Option<CopyIndicator>,
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    pub issue_time: Option<IssueTime>,
    pub reminder_type_code: Option<ReminderTypeCode>,
    pub reminder_sequence_numeric: Option<ReminderSequenceNumeric>,
    pub note: Vec<Note>,
    pub tax_point_date: Option<TaxPointDate>,
    pub document_currency_code: DocumentCurrencyCode,
    pub tax_currency_code: Option<TaxCurrencyCode>,
    pub pricing_currency_code: Option<PricingCurrencyCode>,
    pub payment_currency_code: Option<PaymentCurrencyCode>,
    pub payment_alternative_currency_code: Option<PaymentAlternativeCurrencyCode>,
    pub accounting_cost_code: Option<AccountingCostCode>,
    pub accounting_cost: Option<AccountingCost>,
    pub line_count_numeric: Option<LineCountNumeric>,
    pub reminder_period: Vec<Period>,
    pub additional_document_reference: Vec<DocumentReference>,
    pub signature: Vec<Signature>,
    pub accounting_supplier_party: SupplierParty,
    pub accounting_customer_party: CustomerParty,
    pub payee_party: Option<Party>,
    pub tax_representative_party: Option<Party>,
    pub payment_means: Vec<PaymentMeans>,
    pub payment_terms: Vec<PaymentTerms>,
    pub prepaid_payment: Vec<PrepaidPayment>,
    pub allowance_charge: Vec<AllowanceCharge>,
    pub tax_exchange_rate: Option<ExchangeRate>,
    pub pricing_exchange_rate: Option<ExchangeRate>,
    pub payment_exchange_rate: Option<ExchangeRate>,
    pub payment_alternative_exchange_rate: Option<ExchangeRate>,
    pub tax_total: Vec<TaxTotal>,
    pub legal_monetary_total: Option<LegalTotal>,
    pub reminder_line: Vec<ReminderLine>,
}
