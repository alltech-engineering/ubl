#[derive(Debug, Deserialize, Serialize)]
/// A document to indicate the application's response to a transaction. This may be a business response
/// initiated by a user or a technical response sent automatically by an application.
///
/// UBL Dictionary Entry Name: `Application Response. Details`
///
/// Generated from XSD type `ApplicationResponseType`.
pub struct ApplicationResponse {
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
/// The date on which the information in the response was created.
    #[serde(default, rename = "ResponseDate")]
    pub response_date: Option<udt::DateTime>,
/// The time at which the information in the response was created.
    #[serde(default, rename = "ResponseTime")]
    pub response_time: Option<udt::DateTime>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Identifies the current version of this document.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who sends this Document.
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
/// The Party who receives this Document.
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
/// A response to a document.
    #[serde(default, rename = "DocumentResponse")]
    pub document_response: Vec<cac::DocumentResponse>,
}
