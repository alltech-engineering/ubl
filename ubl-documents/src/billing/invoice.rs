// UBL Invoice — the primary billing document.
// UBL element: Invoice (InvoiceType)
//
// Reference: UBL 2.5 XSD maindoc/UBL-Invoice-2.5.xsd

use serde::{Deserialize, Serialize};
use ubl_common::cac::customer::CustomerParty;
use ubl_common::cac::delivery::Delivery;
use ubl_common::cac::delivery::DeliveryTerms;
use ubl_common::cac::line::InvoiceLine;
use ubl_common::cac::party::Party;
use ubl_common::cac::payment::Payment;
use ubl_common::cac::payment::PaymentMeans;
use ubl_common::cac::payment::PaymentTerms;
use ubl_common::cac::supplier::SupplierParty;

use ubl_common::cac::allowance::AllowanceCharge;
use ubl_common::cac::document::DocumentReference;
use ubl_common::cac::exchange_rate::ExchangeRate;
use ubl_common::cac::order_reference::BillingReference;
use ubl_common::cac::order_reference::OrderReference;
use ubl_common::cac::period::Period;
use ubl_common::cac::tax::TaxTotal;
use ubl_common::cac::totals::LegalTotal;
use ubl_common::cbc::*;

/// A document used to request payment for goods or services.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Invoice {
    // --- Document metadata ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,

    /// The invoice number, assigned by the sender. Required.
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,

    // --- Issue dates ---
    /// The date this invoice was issued. Required.
    pub issue_date: IssueDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<DueDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_point_date: Option<TaxPointDate>,

    // --- Type and notes ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_type_code: Option<InvoiceTypeCode>,
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
    pub originator_customer_party: Option<CustomerParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beneficiary_party: Vec<Party>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prepaid_payment: Vec<Payment>,

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
    /// The legal monetary total for this invoice. Required.
    pub legal_monetary_total: LegalTotal,

    // --- Lines ---
    /// The invoice lines. At least one required.
    #[serde(default)]
    pub invoice_line: Vec<InvoiceLine>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ubl_common::cac;
    use ubl_common::cbc;

    fn make_minimal_invoice() -> Invoice {
        Invoice {
            id: cbc::ID::new("INV-001"),
            uuid: None,
            copy_indicator: None,
            issue_date: cbc::IssueDate::new(chrono::NaiveDate::from_ymd_opt(2026, 6, 12).unwrap()),
            issue_time: None,
            note: vec![],
            tax_point_date: None,
            due_date: None,
            invoice_type_code: None,
            document_currency_code: None,
            tax_currency_code: None,
            pricing_currency_code: None,
            payment_currency_code: None,
            payment_alternative_currency_code: None,
            accounting_cost_code: None,
            accounting_cost: None,
            line_count_numeric: None,
            buyer_reference: None,
            default_language_code: None,
            ubl_version_id: None,
            customization_id: None,
            profile_id: None,
            profile_execution_id: None,
            invoice_period: vec![],
            order_reference: None,
            billing_reference: vec![],
            despatch_document_reference: vec![],
            delivery_note_document_reference: vec![],
            work_report_document_reference: vec![],
            receipt_document_reference: vec![],
            statement_document_reference: vec![],
            originator_document_reference: vec![],
            contract_document_reference: vec![],
            additional_document_reference: vec![],
            accounting_supplier_party: cac::SupplierParty {
                customer_assigned_account_id: None,
                additional_account_id: vec![],
                data_sending_capability: None,
                party: None,
                despatch_contact: None,
                accounting_contact: None,
                seller_contact: None,
            },
            accounting_customer_party: None,
            payee_party: None,
            buyer_customer_party: None,
            seller_supplier_party: None,
            originator_customer_party: None,
            beneficiary_party: vec![],
            tax_representative_party: None,
            delivery: vec![],
            delivery_terms: None,
            payment_means: vec![],
            payment_terms: vec![],
            prepaid_payment: vec![],
            allowance_charge: vec![],
            tax_exchange_rate: None,
            pricing_exchange_rate: None,
            payment_exchange_rate: None,
            payment_alternative_exchange_rate: None,
            tax_total: vec![],
            legal_monetary_total: cac::LegalTotal {
                line_extension_amount: cbc::LineExtensionAmount::new(
                    rust_decimal::Decimal::ZERO,
                    "ZAR",
                ),
                tax_exclusive_amount: None,
                tax_inclusive_amount: None,
                allowance_total_amount: None,
                charge_total_amount: None,
                prepaid_amount: None,
                payable_rounding_amount: None,
                payable_amount: cbc::PayableAmount::new(rust_decimal::Decimal::ZERO, "ZAR"),
            },
            invoice_line: vec![],
        }
    }

    #[test]
    fn test_invoice_roundtrip() {
        let inv = make_minimal_invoice();
        let json = serde_json::to_string(&inv).unwrap();
        let inv2: Invoice = serde_json::from_str(&json).unwrap();
        assert_eq!(inv.id.value(), inv2.id.value());
        assert_eq!(inv.issue_date.0, inv2.issue_date.0);
    }

    #[test]
    fn test_invoice_json_contains_id() {
        let inv = make_minimal_invoice();
        let json = serde_json::to_string(&inv).unwrap();
        assert!(json.contains("INV-001"));
    }
}
