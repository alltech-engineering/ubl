#[derive(Debug, Deserialize, Serialize)]
pub struct EnquiryResponse {
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
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "RequestorParty")]
    pub requestor_party: cac::Party,
    #[serde(rename = "ResponderParty")]
    pub responder_party: cac::Party,
    #[serde(rename = "ParentDocumentReference")]
    pub parent_document_reference: cac::DocumentReference,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Attachment")]
    pub attachment: Vec<cac::Attachment>,
}
