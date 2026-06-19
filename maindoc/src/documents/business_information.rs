#[derive(Debug, Deserialize, Serialize)]
pub struct BusinessInformation {
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
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
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
    #[serde(default, rename = "BriefDescription")]
    pub brief_description: Vec<cct::Text>,
    #[serde(default, rename = "RequestedPublicationDate")]
    pub requested_publication_date: Option<udt::DateTime>,
    #[serde(default, rename = "RegulatoryDomain")]
    pub regulatory_domain: Vec<cct::Text>,
    #[serde(default, rename = "NoticeTypeCode")]
    pub notice_type_code: Option<cct::Code>,
    #[serde(default, rename = "NoticeLanguageCode")]
    pub notice_language_code: Option<cct::Code>,
    #[serde(default, rename = "AdditionalNoticeLanguage")]
    pub additional_notice_language: Vec<cac::Language>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
    #[serde(rename = "BusinessParty")]
    pub business_party: cac::Party,
    #[serde(default, rename = "BrochureDocumentReference")]
    pub brochure_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "BusinessCapability")]
    pub business_capability: Vec<cac::Capability>,
    #[serde(default, rename = "BusinessPartyGroup")]
    pub business_party_group: Vec<cac::PartyGroup>,
    #[serde(default, rename = "OperationType")]
    pub operation_type: Vec<cac::OperationType>,
    #[serde(default, rename = "NoticeSubType")]
    pub notice_sub_type: Option<cac::NoticeSubType>,
}
