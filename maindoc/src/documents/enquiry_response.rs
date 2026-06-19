#[derive(Debug, Deserialize, Serialize)]
/// A document sent by a responder to a requester answering a particular enqury.
///
/// UBL Dictionary Entry Name: `Enquiry Response. Details`
///
/// Generated from XSD type `EnquiryResponseType`.
pub struct EnquiryResponse {
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
/// An identifier for this document, assigned by the responder.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// (Deprecated) Indicates whether this document is a copy (true) or not (false).
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the responder, at which this enquiry response was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the responder, at which this enquiry response was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Free-form text-only enquiry response description pertinent to this document, conveying information
/// that is not contained explicitly in other structures.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who issued the Enquiry.
    #[serde(rename = "RequestorParty")]
    pub requestor_party: cac::Party,
/// The Party who responds to the Enquiry.
    #[serde(rename = "ResponderParty")]
    pub responder_party: cac::Party,
/// Reference to the enquiry that this response refers to.
    #[serde(rename = "ParentDocumentReference")]
    pub parent_document_reference: cac::DocumentReference,
/// References to relevant documents for the response such as the Contract folder or the lot in the
/// eTendering.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// Attachment that includes file-based response.
    #[serde(default, rename = "Attachment")]
    pub attachment: Vec<cac::Attachment>,
}
