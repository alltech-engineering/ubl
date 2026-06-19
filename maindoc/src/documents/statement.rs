#[derive(Debug, Deserialize, Serialize)]
/// A document used to report the status of orders, billing, and payment. This document is a statement
/// of account, not a summary invoice.
///
/// UBL Dictionary Entry Name: `Statement. Details`
///
/// Generated from XSD type `StatementType`.
pub struct Statement {
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
/// The default currency for the Statement.
    #[serde(rename = "DocumentCurrencyCode")]
    pub document_currency_code: cct::Code,
/// The total of all debit amounts for the Statement.
    #[serde(default, rename = "TotalDebitAmount")]
    pub total_debit_amount: Option<cct::Amount>,
/// The total of all credit amounts for the Statement.
    #[serde(default, rename = "TotalCreditAmount")]
    pub total_credit_amount: Option<cct::Amount>,
/// The total amount for the Statement.
    #[serde(default, rename = "TotalBalanceAmount")]
    pub total_balance_amount: Option<cct::Amount>,
/// The number of Statement Lines in the Statement.
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::Numeric>,
/// A code signifying the type of the Statement.
    #[serde(default, rename = "StatementTypeCode")]
    pub statement_type_code: Option<cct::Code>,
/// A period to which the Statement applies.
    #[serde(default, rename = "StatementPeriod")]
    pub statement_period: Option<cac::Period>,
/// A reference to an additional document associated with this document.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The accounting supplier party.
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierParty,
/// The accounting customer party.
    #[serde(rename = "AccountingCustomerParty")]
    pub accounting_customer_party: cac::CustomerParty,
/// The buyer.
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<cac::CustomerParty>,
/// The seller.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
/// The originator.
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Option<cac::CustomerParty>,
/// A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<cac::Party>,
/// The Party who receives the Payment.
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: Option<cac::Party>,
/// Expected means of payment.
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Vec<cac::PaymentMeans>,
/// A set of payment terms associated with this document.
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<cac::PaymentTerms>,
/// A discount or charge that applies to a price component.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<cac::AllowanceCharge>,
/// The total amount of a specific type of tax.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<cac::TaxTotal>,
/// A Statement Line.
    #[serde(default, rename = "StatementLine")]
    pub statement_line: Vec<cac::StatementLine>,
}
