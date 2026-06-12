// UBL OrderResponse document (UBL 2.5)
// A document used to indicate detailed acceptance or rejection of an Order
// or to make a counter-offer.
// Reference: xsd/maindoc/UBL-OrderResponse-2.5.xsd

use serde::{Deserialize, Serialize};
use ubl_common::cbc::*;
use ubl_common::cac::*;
// Disambiguate types present in both cbc and cac, or duplicated within cac.
use ubl_common::cac::exchange_rate::ExchangeRate;
use ubl_common::cac::address::Country;

/// A response to a Purchase Order (line-level detail).
/// UBL element: OrderResponse
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderResponse {
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

    /// Date this response was issued (required).
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_response_code: Option<OrderResponseCode>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,

    // === Currency Codes (BBIE) ===
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_currency_code: Option<DocumentCurrencyCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_code: Option<PricingCurrencyCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_currency_code: Option<TaxCurrencyCode>,

    // === Aggregate Measures (BBIE) ===
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_packages_quantity: Option<TotalPackagesQuantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gross_weight_measure: Option<GrossWeightMeasure>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub net_weight_measure: Option<NetWeightMeasure>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub net_net_weight_measure: Option<NetNetWeightMeasure>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gross_volume_measure: Option<GrossVolumeMeasure>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub net_volume_measure: Option<NetVolumeMeasure>,

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
    /// Reference to the Order being responded to (required, at least 1).
    pub order_reference: Vec<OrderReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order_change_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_document_reference: Option<Box<DocumentReference>>,
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
    pub accounting_supplier_party: Option<SupplierParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_customer_party: Option<CustomerParty>,

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
    pub legal_monetary_total: Option<LegalTotal>,

    // === Response Lines (ASBIE: CAC) ===
    /// Lines in the order response (0..n).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order_line: Vec<OrderLine>,

    // === Beneficiary (ASBIE: CAC) ===
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beneficiary_party: Vec<Party>,
}
