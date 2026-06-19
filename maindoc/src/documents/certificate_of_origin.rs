#[derive(Debug, Deserialize, Serialize)]
/// A document that describes the Certificate of Origin.
///
/// UBL Dictionary Entry Name: `Certificate Of Origin. Details`
///
/// Generated from XSD type `CertificateOfOriginType`.
pub struct CertificateOfOrigin {
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
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Textual description of the document instance.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Identifies the version of this Certificate of Origin.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who makes the export declaration, or on whose behalf the export declaration is made, and
/// who is the owner of the goods or has similar right of disposal over them at the time when the
/// declaration is accepted.
    #[serde(default, rename = "ExporterParty")]
    pub exporter_party: Option<cac::Party>,
/// The Party who imports the goods, or on whose behalf the goods are being imported.
    #[serde(default, rename = "ImporterParty")]
    pub importer_party: Option<cac::Party>,
/// The Party providing the endorsement.
    #[serde(default, rename = "EndorserParty")]
    pub endorser_party: Vec<cac::EndorserParty>,
/// Details of the application for a Certificate of Origin.
    #[serde(rename = "CertificateOfOriginApplication")]
    pub certificate_of_origin_application: cac::CertificateOfOriginApplication,
/// Issuer Endorsement details.
    #[serde(rename = "IssuerEndorsement")]
    pub issuer_endorsement: cac::Endorsement,
/// Embassy Endorsement details.
    #[serde(default, rename = "EmbassyEndorsement")]
    pub embassy_endorsement: Option<cac::Endorsement>,
/// Insurance Endorsement details.
    #[serde(default, rename = "InsuranceEndorsement")]
    pub insurance_endorsement: Option<cac::Endorsement>,
}
