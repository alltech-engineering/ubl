#[derive(Debug, Deserialize, Serialize)]
/// A document used to support business parties agreeing on a set of digital processes, terms and
/// conditions to ensure interoperability.
///
/// UBL Dictionary Entry Name: `Digital Agreement. Details`
///
/// Generated from XSD type `DigitalAgreementType`.
pub struct DigitalAgreement {
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
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// A code signifying the type of digital agreement (e.g. bi-lateral, multi-lateral).
    #[serde(default, rename = "AgreementTypeCode")]
    pub agreement_type_code: Option<cct::Code>,
/// Identifies the current version of this digital agreement.
    #[serde(rename = "VersionID")]
    pub version_id: cct::Identifier,
/// Identifies the previous version of this digital agreement.
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: Option<cct::Identifier>,
/// A code signifying the minimum response message level the parties are required to provide (e.g. EESPA
/// response message level).
    #[serde(default, rename = "RequiredResponseMessageLevelCode")]
    pub required_response_message_level_code: Option<cct::Code>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who governs the Agreement (e.g. a multi-lateral Digital Agreement).
    #[serde(default, rename = "GovernorParty")]
    pub governor_party: Option<cac::Party>,
/// The business parties agreeing on a set of digital processes, terms and conditions to ensure
/// interoperability.
    #[serde(default, rename = "ParticipantParty")]
    pub participant_party: Vec<cac::ParticipantParty>,
/// The country to which this digital agreement applies.
    #[serde(default, rename = "AgreementCountry")]
    pub agreement_country: Vec<cac::Country>,
/// A reference to a certification document required by this digital agreement.
    #[serde(default, rename = "RequiredCertificationDocumentReference")]
    pub required_certification_document_reference: Vec<cac::DocumentReference>,
/// A reference to digital agreement terms and conditions.
    #[serde(default, rename = "DigitalAgreementTerms")]
    pub digital_agreement_terms: Option<cac::DigitalAgreementTerms>,
/// The digital processes in scope of this digital agreement.
    #[serde(default, rename = "DigitalProcess")]
    pub digital_process: Vec<cac::DigitalProcess>,
}
