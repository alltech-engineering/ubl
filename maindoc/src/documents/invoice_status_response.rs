#[derive(Debug, Deserialize, Serialize)]
/// A document used to provide information about the status of a collaboration/process associated with a
/// document.
///
/// UBL Dictionary Entry Name: `Invoice Status Response. Details`
///
/// Generated from XSD type `InvoiceStatusResponseType`.
pub struct InvoiceStatusResponse {
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
/// The date on which this Invoice Status Response was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time at which this Invoice Status Response was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// An identifier for the current version of this Invoice Status Response.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The party sending this document.
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
/// The party receiving this document.
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
/// A Payment associated with one or more Invoices referenced in this Invoice Status Response.
    #[serde(default, rename = "Payment")]
    pub payment: Vec<cac::Payment>,
/// A response indicating the status of an Invoice referenced in this Invoice Status Response.
    #[serde(default, rename = "DocumentResponse")]
    pub document_response: Vec<cac::DocumentResponse>,
}
