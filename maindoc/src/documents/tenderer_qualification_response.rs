#[derive(Debug, Deserialize, Serialize)]
pub struct TendererQualificationResponse {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::IdentifierType,
    #[serde(default, rename = "ContractName")]
    pub contract_name: Vec<cct::TextType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
    #[serde(default, rename = "ResolutionDocumentReference")]
    pub resolution_document_reference: Option<cac::DocumentReference>,
    #[serde(default, rename = "QualificationResolution")]
    pub qualification_resolution: Vec<cac::QualificationResolution>,
    #[serde(default, rename = "AppealTerms")]
    pub appeal_terms: Option<cac::AppealTerms>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
