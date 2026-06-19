#[derive(Debug, Deserialize, Serialize)]
/// A document issued by a procurement organization to notify an economic operator whether it has been
/// admitted to or excluded from the tendering process.
///
/// UBL Dictionary Entry Name: `Tenderer Qualification Response. Details`
///
/// Generated from XSD type `TendererQualificationResponseType`.
pub struct TendererQualificationResponse {
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
/// (Deprecated) Indicates whether this document is a copy (true) or not (false).
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// An identifier, assigned by the sender, for the process file (i.e., record) to which this document
/// belongs.
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::Identifier,
/// Short title of a contract associated with this Tender.
    #[serde(default, rename = "ContractName")]
    pub contract_name: Vec<cct::Text>,
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
/// The Party who sends this message.
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
/// The Party who receives this message.
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
/// A document (e.g., meeting minutes) relating to consideration of tenderer qualifications.
    #[serde(default, rename = "ResolutionDocumentReference")]
    pub resolution_document_reference: Option<cac::DocumentReference>,
/// An association to the resolution that is being notified
    #[serde(default, rename = "QualificationResolution")]
    pub qualification_resolution: Vec<cac::QualificationResolution>,
/// Terms of appeal for this tendering process.
    #[serde(default, rename = "AppealTerms")]
    pub appeal_terms: Option<cac::AppealTerms>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
