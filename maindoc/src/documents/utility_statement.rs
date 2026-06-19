#[derive(Debug, Deserialize, Serialize)]
/// A supplement to an Invoice or Credit Note, containing information on the consumption of services
/// provided by utility suppliers to private and public customers, including electricity, gas, water,
/// and telephone services.
///
/// UBL Dictionary Entry Name: `Utility Statement. Details`
///
/// Generated from XSD type `UtilityStatementType`.
pub struct UtilityStatement {
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
/// A code signifying the type of Utility Statement.
    #[serde(rename = "UtilityStatementTypeCode")]
    pub utility_statement_type_code: cct::Code,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A code signifying the default currency for this document.
    #[serde(rename = "DocumentCurrencyCode")]
    pub document_currency_code: cct::Code,
/// The buyer's accounting cost code, applied to the UtilityStatement.
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
/// The buyer's accounting cost code, applied to the UtilityStatement, expressed as text.
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
/// A reference to the parent Invoice or Credit Note.
    #[serde(rename = "ParentDocumentReference")]
    pub parent_document_reference: cac::DocumentReference,
/// A reference to an additional document associated with this document.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who sends this Utility Statement.
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
/// The Party who receives this Utility Statement.
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
/// The buyer, if different from the receiver of the document.
    #[serde(default, rename = "CustomerParty")]
    pub customer_party: Option<cac::CustomerParty>,
/// The Party that is the subscriber of the utility.
    #[serde(default, rename = "SubscriberParty")]
    pub subscriber_party: Option<cac::Party>,
/// A payment on an account.
    #[serde(default, rename = "MainOnAccountPayment")]
    pub main_on_account_payment: Vec<cac::OnAccountPayment>,
/// A utility statement for a particular consumption point.
    #[serde(default, rename = "SubscriberConsumption")]
    pub subscriber_consumption: Vec<cac::SubscriberConsumption>,
}
