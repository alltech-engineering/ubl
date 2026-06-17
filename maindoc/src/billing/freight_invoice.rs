// UBL FreightInvoice — billing document type.
// UBL element: maindoc:FreightInvoice

use serde::{Deserialize, Serialize};
use ubl_common::cbc::*;
use ubl_common::cac::line::InvoiceLine;
use ubl_common::cac::allowance::AllowanceCharge;
use ubl_common::cac::customer::CustomerParty;
use ubl_common::cac::delivery::Shipment;
use ubl_common::cac::delivery_terms::DeliveryTerms;
use ubl_common::cac::document::DocumentReference;
use ubl_common::cac::document::Signature;
use ubl_common::cac::exchange_rate::ExchangeRate;
use ubl_common::cac::order_reference::{BillingReference, OrderReference};
use ubl_common::cac::party::Party;
use ubl_common::cac::payment::{PaymentMeans, PaymentTerms};
use ubl_common::cac::period::Period;
use ubl_common::cac::prepaid_payment::PrepaidPayment;
use ubl_common::cac::project_reference::ProjectReference;
use ubl_common::cac::supplier::SupplierParty;
use ubl_common::cac::tax::TaxTotal;
use ubl_common::cac::totals::LegalTotal;

/// An invoice for freight/transportation services.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FreightInvoice {
    pub id: ID,
    pub copy_indicator: Option<CopyIndicator>,
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    pub issue_time: Option<IssueTime>,
    pub due_date: Option<DueDate>,
    pub invoice_type_code: Option<InvoiceTypeCode>,
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
    pub invoice_period: Vec<Period>,
    pub shipment: Option<Shipment>,
    pub order_reference: Option<OrderReference>,
    pub billing_reference: Vec<BillingReference>,
    pub despatch_document_reference: Vec<DocumentReference>,
    pub receipt_document_reference: Vec<DocumentReference>,
    pub originator_document_reference: Vec<DocumentReference>,
    pub contract_document_reference: Vec<DocumentReference>,
    pub additional_document_reference: Vec<DocumentReference>,
    pub project_reference: Vec<ProjectReference>,
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
    pub withholding_tax_total: Vec<TaxTotal>,
    pub legal_monetary_total: Option<LegalTotal>,
    pub invoice_line: Vec<InvoiceLine>,
}
