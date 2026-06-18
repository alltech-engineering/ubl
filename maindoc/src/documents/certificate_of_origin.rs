#[derive(Debug, Deserialize, Serialize)]
pub struct CertificateOfOrigin {
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
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(default, rename = "ExporterParty")]
    pub exporter_party: Option<cac::Party>,
    #[serde(default, rename = "ImporterParty")]
    pub importer_party: Option<cac::Party>,
    #[serde(default, rename = "EndorserParty")]
    pub endorser_party: Vec<cac::EndorserParty>,
    #[serde(rename = "CertificateOfOriginApplication")]
    pub certificate_of_origin_application: cac::CertificateOfOriginApplication,
    #[serde(rename = "IssuerEndorsement")]
    pub issuer_endorsement: cac::Endorsement,
    #[serde(default, rename = "EmbassyEndorsement")]
    pub embassy_endorsement: Option<cac::Endorsement>,
    #[serde(default, rename = "InsuranceEndorsement")]
    pub insurance_endorsement: Option<cac::Endorsement>,
}
