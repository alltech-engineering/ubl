#[derive(Debug, Deserialize, Serialize)]
pub struct DigitalAgreement {
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
    #[serde(default, rename = "AgreementTypeCode")]
    pub agreement_type_code: Option<cct::Code>,
    #[serde(rename = "VersionID")]
    pub version_id: cct::Identifier,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: Option<cct::Identifier>,
    #[serde(default, rename = "RequiredResponseMessageLevelCode")]
    pub required_response_message_level_code: Option<cct::Code>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(default, rename = "GovernorParty")]
    pub governor_party: Option<cac::Party>,
    #[serde(default, rename = "ParticipantParty")]
    pub participant_party: Vec<cac::ParticipantParty>,
    #[serde(default, rename = "AgreementCountry")]
    pub agreement_country: Vec<cac::Country>,
    #[serde(default, rename = "RequiredCertificationDocumentReference")]
    pub required_certification_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "DigitalAgreementTerms")]
    pub digital_agreement_terms: Option<cac::DigitalAgreementTerms>,
    #[serde(default, rename = "DigitalProcess")]
    pub digital_process: Vec<cac::DigitalProcess>,
}
