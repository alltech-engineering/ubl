#[derive(Debug, Deserialize, Serialize)]
/// A document declaring the qualifications of a tenderer.
///
/// UBL Dictionary Entry Name: `Tenderer Qualification. Details`
///
/// Generated from XSD type `TendererQualificationType`.
pub struct TendererQualification {
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
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Indicates the current version of the Tenderer Qualification.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// Identifies the previous version of the Tenderer Qualification which is superceded by this version.
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: Option<cct::Identifier>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// A specific qualification of the Tenderer.
    #[serde(default, rename = "TendererPartyQualification")]
    pub tenderer_party_qualification: Vec<cac::TendererPartyQualification>,
/// The contracting party.
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: Option<cac::ContractingParty>,
/// An evidentiary document supporting Tenderer qualifications.
    #[serde(default, rename = "Evidence")]
    pub evidence: Vec<cac::Evidence>,
/// A reference to an additional document associated with this document.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
}
