#[derive(Debug, Deserialize, Serialize)]
pub struct TenderContract {
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
    #[serde(default, rename = "RegulatoryDomain")]
    pub regulatory_domain: Vec<cct::Text>,
    #[serde(default, rename = "PublishAwardIndicator")]
    pub publish_award_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "PreviousDocumentReference")]
    pub previous_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: Vec<cac::ContractingParty>,
    #[serde(default, rename = "EconomicOperatorParty")]
    pub economic_operator_party: Vec<cac::EconomicOperatorParty>,
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
