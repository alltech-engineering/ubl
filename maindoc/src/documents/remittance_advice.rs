#[derive(Debug, Deserialize, Serialize)]
/// A document that specifies details of an actual payment.
///
/// UBL Dictionary Entry Name: `Remittance Advice. Details`
///
/// Generated from XSD type `RemittanceAdviceType`.
pub struct RemittanceAdvice {
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
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A code signifying the default currency for this document.
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: Option<cct::Code>,
/// The totals of all debit amounts for the Remittance Advice.
    #[serde(default, rename = "TotalDebitAmount")]
    pub total_debit_amount: Option<cct::Amount>,
/// The totals of all credit amounts for the Remittance Advice.
    #[serde(default, rename = "TotalCreditAmount")]
    pub total_credit_amount: Option<cct::Amount>,
/// The total payable amount for the Remittance Advice (must be positive).
    #[serde(default, rename = "TotalPaymentAmount")]
    pub total_payment_amount: Option<cct::Amount>,
/// An internal reference to the order for payment from the payer to the payer's bank.
    #[serde(default, rename = "PaymentOrderReference")]
    pub payment_order_reference: Option<cct::Text>,
/// An internal reference to the payer's order for payment.
    #[serde(default, rename = "PayerReference")]
    pub payer_reference: Option<cct::Text>,
/// An internal reference to the order for payment by the invoicing party. This may have been requested
/// of the payer by the payee to accompany the payer's remittance.
    #[serde(default, rename = "InvoicingPartyReference")]
    pub invoicing_party_reference: Option<cct::Text>,
/// The number of Remittance Advice Lines in the document.
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::Numeric>,
/// A period (rather than a specific invoice) associated with this document.
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: Vec<cac::Period>,
/// A reference to a billing document associated with this document.
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Option<cac::BillingReference>,
/// A reference to an additional document associated with this document.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
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
/// Expected means of payment.
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Option<cac::PaymentMeans>,
/// The total amount of a specific type of tax.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<cac::TaxTotal>,
/// A line specifying a balance.
    #[serde(default, rename = "RemittanceAdviceLine")]
    pub remittance_advice_line: Vec<cac::RemittanceAdviceLine>,
}
