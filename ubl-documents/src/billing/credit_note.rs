// UBL CreditNote — reduces a previously issued invoice.
// UBL element: CreditNote (CreditNoteType)
//
// Reference: UBL 2.5 XSD maindoc/UBL-CreditNote-2.5.xsd

use serde::{Deserialize, Serialize};
use ubl_common::cac::line::CreditNoteLine;
use ubl_common::cac::discrepancy_response::DiscrepancyResponse;
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

/// A document used to reduce the amount of a previously issued invoice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditNote {
    // --- Document metadata ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,

    /// The credit note number, assigned by the sender. Required.
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,

    // --- Issue dates ---
    /// The date this credit note was issued. Required.
    pub issue_date: IssueDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<DueDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_point_date: Option<TaxPointDate>,

    // --- Type and notes ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_note_type_code: Option<CreditNoteTypeCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,

    // --- Currency codes ---
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

    // --- Accounting ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<AccountingCostCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<AccountingCost>,

    // --- Counts ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_count_numeric: Option<LineCountNumeric>,

    // --- Deprecated ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_reference: Option<BuyerReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_language_code: Option<DefaultLanguageCode>,

    // --- Periods ---
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub invoice_period: Vec<Period>,

    // --- Discrepancy ---
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub discrepancy_response: Vec<DiscrepancyResponse>,

    // --- References ---
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

    // --- Parties ---
    /// The supplier party. Required.
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
    pub tax_representative_party: Option<Party>,

    // --- Delivery ---
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delivery: Vec<Delivery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<DeliveryTerms>,

    // --- Payment ---
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payment_means: Vec<PaymentMeans>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payment_terms: Vec<PaymentTerms>,

    // --- Allowances & Charges ---
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowance_charge: Vec<AllowanceCharge>,

    // --- Exchange Rates ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exchange_rate: Option<ExchangeRate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_exchange_rate: Option<ExchangeRate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_exchange_rate: Option<ExchangeRate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_alternative_exchange_rate: Option<ExchangeRate>,

    // --- Tax ---
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tax_total: Vec<TaxTotal>,

    // --- Totals ---
    /// The legal monetary total for this credit note. Required.
    pub legal_monetary_total: LegalTotal,

    // --- Lines ---
    /// The credit note lines. At least one required.
    #[serde(default)]
    pub credit_note_line: Vec<CreditNoteLine>,
}
