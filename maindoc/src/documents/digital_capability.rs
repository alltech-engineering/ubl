#[derive(Debug, Deserialize, Serialize)]
/// A document used to provide information about a business party and its digital trade capabilities.
///
/// UBL Dictionary Entry Name: `Digital Capability. Details`
///
/// Generated from XSD type `DigitalCapabilityType`.
pub struct DigitalCapability {
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
/// Identifies the current version of party's digital capabilities.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// Identifies the previous version of party's digital capabilities.
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: Option<cct::Identifier>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who sends these Digital Capabilities. This Party may be the owner of these Digital
/// Capabilities or a third-Party who acts on behalf of the owner (e.g. service provider).
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
/// The Party who receives these Digital Capabilities.
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
/// The Party who owns these Digital Capabilities.
    #[serde(rename = "BusinessParty")]
    pub business_party: cac::Party,
/// The digital trade processes supported by the party.
    #[serde(default, rename = "DigitalProcess")]
    pub digital_process: Vec<cac::DigitalProcess>,
}
