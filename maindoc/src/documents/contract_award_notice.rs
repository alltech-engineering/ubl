#[derive(Debug, Deserialize, Serialize)]
pub struct ContractAwardNotice {
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
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "RequestedPublicationDate")]
    pub requested_publication_date: Option<udt::DateTimeType>,
    #[serde(default, rename = "RegulatoryDomain")]
    pub regulatory_domain: Vec<cct::TextType>,
    #[serde(default, rename = "NoticeTypeCode")]
    pub notice_type_code: Option<cct::CodeType>,
    #[serde(default, rename = "PublishAwardIndicator")]
    pub publish_award_indicator: Option<udt::IndicatorType>,
    #[serde(default, rename = "NoticeLanguageCode")]
    pub notice_language_code: Option<cct::CodeType>,
    #[serde(default, rename = "AdditionalNoticeLanguage")]
    pub additional_notice_language: Vec<cac::Language>,
    #[serde(default, rename = "PreviousDocumentReference")]
    pub previous_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "MinutesDocumentReference")]
    pub minutes_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: Vec<cac::ContractingParty>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<cac::Party>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
    #[serde(default, rename = "TenderingTerms")]
    pub tendering_terms: Option<cac::TenderingTerms>,
    #[serde(default, rename = "TenderingProcess")]
    pub tendering_process: Option<cac::TenderingProcess>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: Option<cac::ProcurementProject>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: Vec<cac::ProcurementProjectLot>,
    #[serde(default, rename = "TenderResult")]
    pub tender_result: Vec<cac::TenderResult>,
}
