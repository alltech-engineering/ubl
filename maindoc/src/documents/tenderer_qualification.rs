#[derive(Debug, Deserialize, Serialize)]
pub struct TendererQualification {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::Identifier,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: Option<cct::Identifier>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(default, rename = "TendererPartyQualification")]
    pub tenderer_party_qualification: Vec<cac::TendererPartyQualification>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: Option<cac::ContractingParty>,
    #[serde(default, rename = "Evidence")]
    pub evidence: Vec<cac::Evidence>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
}
