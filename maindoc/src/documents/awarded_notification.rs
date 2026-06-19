#[derive(Debug, Deserialize, Serialize)]
/// The document used to communicate a contract award to the winner.
///
/// UBL Dictionary Entry Name: `Awarded Notification. Details`
///
/// Generated from XSD type `AwardedNotificationType`.
pub struct AwardedNotification {
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
/// Indicates whether this document is a copy (true) or not (false).
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// An identifier, assigned by the sender, for the process file (i.e., record) to which this document
/// belongs.
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::Identifier,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// The name, expressed as text, of this procurement project.
    #[serde(default, rename = "ContractName")]
    pub contract_name: Vec<cct::Text>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The Party who sends this Document.
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
/// The Party who receives this Document.
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
/// A reference to a set of minutes associated with this award.
    #[serde(default, rename = "MinutesDocumentReference")]
    pub minutes_document_reference: Option<cac::DocumentReference>,
/// A reference to an additional document associated with this document.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// The result of the tendering process reported in this notification.
    #[serde(default, rename = "TenderResult")]
    pub tender_result: Vec<cac::TenderResult>,
/// A bond guarantee by the submitter of a tender or bid, required of the tender winner.
    #[serde(default, rename = "FinalFinancialGuarantee")]
    pub final_financial_guarantee: Vec<cac::FinancialGuarantee>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
