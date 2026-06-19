#[derive(Debug, Deserialize, Serialize)]
pub struct TenderReceipt {
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
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
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
    #[serde(default, rename = "ContractName")]
    pub contract_name: Vec<cct::Text>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(rename = "RegisteredDate")]
    pub registered_date: udt::DateTime,
    #[serde(rename = "RegisteredTime")]
    pub registered_time: udt::DateTime,
    #[serde(default, rename = "TenderDocumentReference")]
    pub tender_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
}
