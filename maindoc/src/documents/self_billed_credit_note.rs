#[derive(Debug, Deserialize, Serialize)]
/// A credit note created by the debtor in a self billing arrangement with a creditor.
///
/// UBL Dictionary Entry Name: `Self Billed Credit Note. Details`
///
/// Generated from XSD type `SelfBilledCreditNoteType`.
pub struct SelfBilledCreditNote {
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
/// The date on which SelfBilledCreditNote is due.
    #[serde(default, rename = "DueDate")]
    pub due_date: Option<udt::DateTime>,
/// The date of the Self Billed Credit Note, used to indicate the point at which tax becomes applicable.
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: Option<udt::DateTime>,
/// A code signifying the type of Selfbilled CreditNote
    #[serde(default, rename = "CreditNoteTypeCode")]
    pub credit_note_type_code: Option<cct::Code>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A code signifying the default currency for this document.
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: Option<cct::Code>,
/// A code signifying the currency used for tax amounts in the Self Billed Credit Note.
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: Option<cct::Code>,
/// A code signifying the currency used for prices in the Self Billed Credit Note.
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: Option<cct::Code>,
/// A code signifying the currency used for payment in the Self Billed Credit Note.
    #[serde(default, rename = "PaymentCurrencyCode")]
    pub payment_currency_code: Option<cct::Code>,
/// A code signifying the alternative currency used for payment in the Self Billed Credit Note.
    #[serde(default, rename = "PaymentAlternativeCurrencyCode")]
    pub payment_alternative_currency_code: Option<cct::Code>,
/// The buyer's accounting code, applied to the Self Billed Credit Note as a whole.
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
/// The buyer's accounting code, applied to the Self Billed Credit Note as a whole, expressed as text.
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
/// The number of Self Billed Credit Note Lines in this document.
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::Numeric>,
/// (Deprecated) A reference provided by the buyer used for internal routing of the document.
    #[serde(default, rename = "BuyerReference")]
    pub buyer_reference: Option<cct::Text>,
/// A code signifying the default natural language used by the sender for human-readable textual content
/// that does not include a languageID.
    #[serde(default, rename = "DefaultLanguageCode")]
    pub default_language_code: Option<cct::Code>,
/// A period (rather than a specific Invoice) associated with the Self Billed Credit Note.
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: Vec<cac::Period>,
/// A reason for the Self Billed Credit Note as a whole.
    #[serde(default, rename = "DiscrepancyResponse")]
    pub discrepancy_response: Vec<cac::Response>,
/// The Order associated with this Self Billed Credit Note.
    #[serde(default, rename = "OrderReference")]
    pub order_reference: Option<cac::OrderReference>,
/// A reference to a billing document associated with this document.
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<cac::BillingReference>,
/// A reference to a Despatch Advice associated with this document.
    #[serde(default, rename = "DespatchDocumentReference")]
    pub despatch_document_reference: Vec<cac::DocumentReference>,
/// A reference to a Delivery Note associated with this document.
    #[serde(default, rename = "DeliveryNoteDocumentReference")]
    pub delivery_note_document_reference: Vec<cac::DocumentReference>,
/// A reference to a Work Report associated with this document.
    #[serde(default, rename = "WorkReportDocumentReference")]
    pub work_report_document_reference: Vec<cac::DocumentReference>,
/// A reference to a Receipt Advice associated with this document.
    #[serde(default, rename = "ReceiptDocumentReference")]
    pub receipt_document_reference: Vec<cac::DocumentReference>,
/// A reference to a contract associated with this document.
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: Vec<cac::DocumentReference>,
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
/// etc., to which this Credit Note relates.
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: Vec<cac::PurchaseReference>,
/// A structured annotation providing contextual or explanatory information related to this Credit Note.
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
/// The Party who receives the Payment.
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: Option<cac::Party>,
/// The buyer.
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<cac::CustomerParty>,
/// The seller.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
/// The Party authorized to act as the Tax Representative for this Self Billed Credit Note.
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
    pub payment_means: Vec<cac::PaymentMeans>,
/// A set of payment terms associated with this document.
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<cac::PaymentTerms>,
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
/// The total amount payable on the Self Billed Credit Note, including Allowances, Charges, and Taxes.
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: cac::MonetaryTotal,
/// A line describing an item or amount collected on behalf of a third party.
    #[serde(default, rename = "CollectionCreditNoteLine")]
    pub collection_credit_note_line: Vec<cac::CreditNoteLine>,
/// A Self Billed Credit Note Line.
    #[serde(default, rename = "CreditNoteLine")]
    pub credit_note_line: Vec<cac::CreditNoteLine>,
}
