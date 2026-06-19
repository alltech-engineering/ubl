#[derive(Debug, Deserialize, Serialize)]
/// A document used to specify changes to an existing Order.
///
/// UBL Dictionary Entry Name: `Order Change. Details`
///
/// Generated from XSD type `OrderChangeType`.
pub struct OrderChange {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
/// Identifies the earliest version of the UBL 2 schema for this document type that defines all of the
/// elements that might be encountered in the current instance.
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
/// Identifies a user-defined customization of UBL for a specific use.
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
/// Identifies a user-defined profile of the customization of UBL being used.
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
/// Identifies an instance of executing a profile, to associate all transactions in a collaboration.
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
/// An identifier for this document, assigned by the sender.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// An identifier for the Order Change, assigned by the seller.
    #[serde(default, rename = "SalesOrderID")]
    pub sales_order_id: Option<cct::Identifier>,
/// (Deprecated) Indicates whether this document is a copy (true) or not (false).
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// The Order Change Sequence Number assigned by the Buyer to ensure the proper sequencing of changes.
    #[serde(rename = "SequenceNumberID")]
    pub sequence_number_id: cct::Identifier,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A code signifying he currency requested for amount totals in Invoices related to this Order Change.
    #[serde(default, rename = "RequestedInvoiceCurrencyCode")]
    pub requested_invoice_currency_code: Option<cct::Code>,
/// A code signifying the default currency for this document.
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: Option<cct::Code>,
/// A code signifying the currency that is used for all prices in the Order Change.
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: Option<cct::Code>,
/// A code signifying the currency requested for tax amounts in Invoices related to this Order Change.
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: Option<cct::Code>,
/// A supplementary reference for the transaction (e.g., CRI when using purchasing card).
    #[serde(default, rename = "CustomerReference")]
    pub customer_reference: Option<cct::Text>,
/// The buyer's accounting code, applied to the Order Change as a whole.
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
/// The buyer's accounting code, applied to the Order Change as a whole, expressed as text.
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
/// The number of Order Change lines in the document.
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::Numeric>,
/// A period during which the Order Change is valid.
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<cac::Period>,
/// A reference to the Order being changed.
    #[serde(rename = "OrderReference")]
    pub order_reference: cac::OrderReference,
/// A reference to a Quotation.
    #[serde(default, rename = "QuotationDocumentReference")]
    pub quotation_document_reference: Option<cac::DocumentReference>,
/// A reference to an originator document associated with this document.
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: Option<cac::DocumentReference>,
/// A reference to an additional document associated with this document.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// A contract associated with the Order being changed.
    #[serde(default, rename = "Contract")]
    pub contract: Vec<cac::Contract>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The buyer.
    #[serde(rename = "BuyerCustomerParty")]
    pub buyer_customer_party: cac::CustomerParty,
/// The seller.
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierParty,
/// The originator.
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Option<cac::CustomerParty>,
/// A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<cac::Party>,
/// A freight forwarder or carrier.
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: Option<cac::Party>,
/// The accounting customer party.
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: Option<cac::CustomerParty>,
/// The accounting supplier party.
    #[serde(default, rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: Option<cac::SupplierParty>,
/// A delivery associated with this document.
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<cac::Delivery>,
/// A set of delivery terms associated with this document.
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Option<cac::DeliveryTerms>,
/// Expected means of payment.
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Vec<cac::PaymentMeans>,
/// A set of payment terms associated with this document.
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<cac::PaymentTerms>,
/// Purchasing, sales, or payment conditions applying to the whole Order being changed.
    #[serde(default, rename = "TransactionConditions")]
    pub transaction_conditions: Option<cac::TransactionConditions>,
/// A discount or charge that applies to a price component.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<cac::AllowanceCharge>,
/// The exchange rate between the document currency and the tax currency.
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: Option<cac::ExchangeRate>,
/// The exchange rate between the document currency and the pricing currency.
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: Option<cac::ExchangeRate>,
/// The exchange rate between the document currency and the payment currency.
    #[serde(default, rename = "PaymentExchangeRate")]
    pub payment_exchange_rate: Option<cac::ExchangeRate>,
/// The country of destination (for customs purposes).
    #[serde(default, rename = "DestinationCountry")]
    pub destination_country: Option<cac::Country>,
/// The total amount of a specific type of tax.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<cac::TaxTotal>,
/// The amount of change to the total cost of the order anticipated by the buyer.
    #[serde(default, rename = "AnticipatedMonetaryTotal")]
    pub anticipated_monetary_total: Option<cac::MonetaryTotal>,
/// An association to one or more (changed) Order Lines.
    #[serde(default, rename = "OrderLine")]
    pub order_line: Vec<cac::OrderLine>,
}
