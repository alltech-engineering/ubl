// UBL Order document (UBL 2.5)
// A document used to order goods and services.
// Reference: xsd/maindoc/UBL-Order-2.5.xsd

use serde::{Deserialize, Serialize};
use ubl_common::cac::*;
use ubl_common::cbc::*;
// Disambiguate types present in both cbc and cac, or duplicated within cac.
use ubl_common::cac::address::Country;
use ubl_common::cac::exchange_rate::ExchangeRate;

/// A Purchase Order.
/// UBL element: Order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    // === Document Metadata (BBIE) ===
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,

    /// Sender-assigned document identifier (required).
    pub id: ID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sales_order_id: Option<SalesOrderID>,
    #[deprecated(note = "Deprecated in UBL 2.5")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,

    /// Date this order was issued (required).
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_type_code: Option<OrderTypeCode>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,

    // === Currency Codes (BBIE) ===
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_invoice_currency_code: Option<RequestedInvoiceCurrencyCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_currency_code: Option<DocumentCurrencyCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_code: Option<PricingCurrencyCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_currency_code: Option<TaxCurrencyCode>,

    // === Accounting (BBIE) ===
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<CustomerReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<AccountingCostCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<AccountingCost>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_count_numeric: Option<LineCountNumeric>,

    // === Document References (ASBIE: CAC) ===
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validity_period: Vec<Period>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quotation_document_reference: Option<Box<DocumentReference>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_document_reference: Option<Box<DocumentReference>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalogue_reference: Option<CatalogueReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<DocumentReference>,

    // === Contract & Signature (ASBIE: CAC) ===
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract: Vec<Contract>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,

    // === Parties (ASBIE: CAC) ===
    /// The buyer (required).
    pub buyer_customer_party: CustomerParty,
    /// The seller (required).
    pub seller_supplier_party: SupplierParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_customer_party: Option<CustomerParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freight_forwarder_party: Option<Party>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_customer_party: Option<CustomerParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_supplier_party: Option<SupplierParty>,

    // === Delivery (ASBIE: CAC) ===
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delivery: Vec<Delivery>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delivery_terms: Vec<DeliveryTerms>,

    // === Payment (ASBIE: CAC) ===
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payment_means: Vec<PaymentMeans>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payment_terms: Vec<PaymentTerms>,

    // === Financial (ASBIE: CAC) ===
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_conditions: Option<TransactionConditions>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_exchange_rate: Option<ExchangeRate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_exchange_rate: Option<ExchangeRate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_exchange_rate: Option<ExchangeRate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_country: Option<Country>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tax_total: Vec<TaxTotal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anticipated_monetary_total: Option<MonetaryTotal>,

    // === Order Lines (ASBIE: CAC) ===
    /// The order lines (at least one required).
    pub order_line: Vec<OrderLine>,

    // === Project & Beneficiary (ASBIE: CAC) ===
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub project_reference: Vec<ProjectReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beneficiary_party: Vec<Party>,
}
