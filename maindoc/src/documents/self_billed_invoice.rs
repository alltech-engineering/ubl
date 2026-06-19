#[derive(Debug, Deserialize, Serialize)]
/// An Invoice document created by the Customer (rather than the Supplier) in a Self Billing
/// relationship.
///
/// UBL Dictionary Entry Name: `Self Billed Invoice. Details`
///
/// Generated from XSD type `SelfBilledInvoiceType`.
pub struct SelfBilledInvoice {
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
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
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
/// The date on which Invoice is due.
    #[serde(default, rename = "DueDate")]
    pub due_date: Option<udt::DateTime>,
/// The date of the invoice for tax purposes, in accordance with the applicable tax regulation.
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: Option<udt::DateTime>,
/// A code signifying the type of invoice.
    #[serde(default, rename = "InvoiceTypeCode")]
    pub invoice_type_code: Option<cct::Code>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A code signifying the default currency for this document.
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: Option<cct::Code>,
/// A code signifying the currency used for tax amounts in the Invoice.
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: Option<cct::Code>,
/// A code signifying the currency used for prices in the Invoice.
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: Option<cct::Code>,
/// A code signifying the currency used for payment in the Invoice.
    #[serde(default, rename = "PaymentCurrencyCode")]
    pub payment_currency_code: Option<cct::Code>,
/// A code signifying the alternative currency used for payment in the Invoice.
    #[serde(default, rename = "PaymentAlternativeCurrencyCode")]
    pub payment_alternative_currency_code: Option<cct::Code>,
/// An accounting cost code, applied to the Invoice as a whole.
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
/// An accounting cost code, applied to the Invoice as a whole, expressed as text.
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
/// The number of Invoice Lines in this document.
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::Numeric>,
/// (Deprecated) A reference provided by the buyer used for internal routing of the document.
    #[serde(default, rename = "BuyerReference")]
    pub buyer_reference: Option<cct::Text>,
/// A code signifying the default natural language used by the sender for human-readable textual content
/// that does not include a languageID.
    #[serde(default, rename = "DefaultLanguageCode")]
    pub default_language_code: Option<cct::Code>,
/// A period to which the Self Billed Invoice applies.
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: Vec<cac::Period>,
/// A reference to the Order with which this Invoice is associated.
    #[serde(default, rename = "OrderReference")]
    pub order_reference: Option<cac::OrderReference>,
/// A reference to a billing document associated with this document.
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<cac::BillingReference>,
/// A reference to a contract associated with this document.
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: Vec<cac::DocumentReference>,
/// A reference to a Despatch Advice associated with this document.
    #[serde(default, rename = "DespatchDocumentReference")]
    pub despatch_document_reference: Vec<cac::DocumentReference>,
/// A reference to a Work Report associated with this document.
    #[serde(default, rename = "WorkReportDocumentReference")]
    pub work_report_document_reference: Vec<cac::DocumentReference>,
/// A reference to a Delivery Note associated with this document.
    #[serde(default, rename = "DeliveryNoteDocumentReference")]
    pub delivery_note_document_reference: Vec<cac::DocumentReference>,
/// A reference to a Receipt Advice associated with this document.
    #[serde(default, rename = "ReceiptDocumentReference")]
    pub receipt_document_reference: Vec<cac::DocumentReference>,
/// A reference to a Statement associated with this document.
    #[serde(default, rename = "StatementDocumentReference")]
    pub statement_document_reference: Vec<cac::DocumentReference>,
/// A reference to an originator document associated with this document.
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: Vec<cac::DocumentReference>,
/// A reference to an additional document associated with this document.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// A reference to a project associated with this document.
    #[serde(default, rename = "ProjectReference")]
    pub project_reference: Vec<cac::ProjectReference>,
/// A reference provided by the buyer used for internal routing of the document.
    #[serde(default, rename = "BuyerAssignedReference")]
    pub buyer_assigned_reference: Vec<cac::BuyerAssignedReference>,
/// A reference to an object, such as a subscription number, telephone number, meter, vehicle, person,
/// etc., to which this Invoice relates.
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: Vec<cac::PurchaseReference>,
/// A structured annotation providing contextual or explanatory information related to this Invoice.
    #[serde(default, rename = "Annotation")]
    pub annotation: Vec<cac::Annotation>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The accounting customer party.
    #[serde(rename = "AccountingCustomerParty")]
    pub accounting_customer_party: cac::CustomerParty,
/// The accounting supplier party.
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierParty,
/// The buyer.
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<cac::CustomerParty>,
/// The seller.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
/// The Party who originated the Order to which this Invoice is related.
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Option<cac::CustomerParty>,
/// A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<cac::Party>,
/// The Party who receives the Payment.
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: Option<cac::Party>,
/// The Party authorized to act as the Tax Representative for this Self Billed Invoice.
    #[serde(default, rename = "TaxRepresentativeParty")]
    pub tax_representative_party: Option<cac::Party>,
/// A delivery associated with this document.
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<cac::Delivery>,
/// A set of delivery terms associated with this document.
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Vec<cac::DeliveryTerms>,
/// Expected means of payment.
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Option<cac::PaymentMeans>,
/// A set of payment terms associated with this document.
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<cac::PaymentTerms>,
/// A prepaid payment.
    #[serde(default, rename = "PrepaidPayment")]
    pub prepaid_payment: Vec<cac::Payment>,
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
/// The exchange rate between the document currency and the payment alternative currency.
    #[serde(default, rename = "PaymentAlternativeExchangeRate")]
    pub payment_alternative_exchange_rate: Option<cac::ExchangeRate>,
/// The total amount of a specific type of tax.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<cac::TaxTotal>,
/// The total withholding tax.
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: Vec<cac::TaxTotal>,
/// A set of totals associated with this Invoice that are required for the Invoice to be a legal
/// document.
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: cac::MonetaryTotal,
/// A line describing an item or amount collected on behalf of a third party.
    #[serde(default, rename = "CollectionInvoiceLine")]
    pub collection_invoice_line: Vec<cac::InvoiceLine>,
/// A line describing an Invoice Item.
    #[serde(default, rename = "InvoiceLine")]
    pub invoice_line: Vec<cac::InvoiceLine>,
}
