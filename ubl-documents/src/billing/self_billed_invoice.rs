// UBL SelfBilledInvoice — an invoice issued by the buyer rather than the supplier.
// UBL element: SelfBilledInvoice (InvoiceType — same schema as Invoice)
//
// Reference: UBL 2.5 XSD maindoc/UBL-SelfBilledInvoice-2.5.xsd
//
// Structurally identical to Invoice. The semantic distinction (buyer-issued) is
// captured by the type name. In UBL 2.5, SelfBilledInvoice reuses the InvoiceType
// schema directly.

use serde::{Deserialize, Serialize};
use ubl_common::cac::line::InvoiceLine;
use ubl_common::cac::party::Party;
use ubl_common::cac::supplier::SupplierParty;
use ubl_common::cac::customer::CustomerParty;
use ubl_common::cac::delivery::Delivery;
use ubl_common::cac::delivery::DeliveryTerms;
use ubl_common::cac::payment::PaymentMeans;
use ubl_common::cac::payment::PaymentTerms;
use ubl_common::cac::payment::Payment;

use ubl_common::cac::allowance::AllowanceCharge;
use ubl_common::cac::exchange_rate::ExchangeRate;
use ubl_common::cac::tax::TaxTotal;
use ubl_common::cac::totals::LegalTotal;
use ubl_common::cac::order_reference::OrderReference;
use ubl_common::cac::order_reference::BillingReference;
use ubl_common::cac::document::DocumentReference;
use ubl_common::cac::period::Period;
use ubl_common::cbc::*;

/// An invoice issued by the buyer. Structurally identical to Invoice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfBilledInvoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<DueDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_point_date: Option<TaxPointDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_type_code: Option<InvoiceTypeCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_currency_code: Option<DocumentCurrencyCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_currency_code: Option<TaxCurrencyCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_currency_code: Option<PricingCurrencyCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_currency_code: Option<PaymentCurrencyCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_alternative_currency_code: Option<PaymentAlternativeCurrencyCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<AccountingCostCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<AccountingCost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_count_numeric: Option<LineCountNumeric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_reference: Option<BuyerReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_language_code: Option<DefaultLanguageCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub invoice_period: Vec<Period>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_reference: Option<OrderReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub billing_reference: Vec<BillingReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub despatch_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delivery_note_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub work_report_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub receipt_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statement_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub originator_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<DocumentReference>,
    pub accounting_supplier_party: SupplierParty,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_customer_party: Option<CustomerParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_party: Option<Party>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_customer_party: Option<CustomerParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_supplier_party: Option<SupplierParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_customer_party: Option<CustomerParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beneficiary_party: Vec<Party>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_representative_party: Option<Party>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delivery: Vec<Delivery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<DeliveryTerms>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payment_means: Vec<PaymentMeans>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payment_terms: Vec<PaymentTerms>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prepaid_payment: Vec<Payment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exchange_rate: Option<ExchangeRate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_exchange_rate: Option<ExchangeRate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_exchange_rate: Option<ExchangeRate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_alternative_exchange_rate: Option<ExchangeRate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tax_total: Vec<TaxTotal>,
    pub legal_monetary_total: LegalTotal,
    #[serde(default)]
    pub invoice_line: Vec<InvoiceLine>,
}
