#[derive(Debug, Deserialize, Serialize)]
pub struct PriorInformationNotice {
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
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: Option<cct::Identifier>,
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
    #[serde(default, rename = "RequestedPublicationDate")]
    pub requested_publication_date: Option<udt::DateTime>,
    #[serde(default, rename = "PlannedDate")]
    pub planned_date: Option<udt::DateTime>,
    #[serde(default, rename = "RegulatoryDomain")]
    pub regulatory_domain: Vec<cct::Text>,
    #[serde(default, rename = "NoticeTypeCode")]
    pub notice_type_code: Option<cct::Code>,
    #[serde(default, rename = "NoticeLanguageCode")]
    pub notice_language_code: Option<cct::Code>,
    #[serde(default, rename = "AdditionalNoticeLanguage")]
    pub additional_notice_language: Vec<cac::Language>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: Vec<cac::ContractingParty>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Vec<cac::CustomerParty>,
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
}
