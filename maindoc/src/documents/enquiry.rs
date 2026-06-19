#[derive(Debug, Deserialize, Serialize)]
/// A document sent by a requestor to a responder resquesting information about a particular business
/// process.
///
/// UBL Dictionary Entry Name: `Enquiry. Details`
///
/// Generated from XSD type `EnquiryType`.
pub struct Enquiry {
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
/// An identifier for this document, assigned by the requestor.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// (Deprecated) Indicates whether this document is a copy (true) or not (false).
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the requestor, at which this enquiry was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the requestor, at which this enquiry was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// The date, assigned by the requestor, by which this enquiry will be replied.
    #[serde(default, rename = "LatestReplyDate")]
    pub latest_reply_date: Option<udt::DateTime>,
/// The time, assigned by the requestor, by which this enquiry will be replied.
    #[serde(default, rename = "LatestReplyTime")]
    pub latest_reply_time: Option<udt::DateTime>,
/// Free-form text-only description pertinent to this document, conveying information that is not
/// contained explicitly in other structures.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who issues this Enquiry.
    #[serde(rename = "RequestorParty")]
    pub requestor_party: cac::Party,
/// The Party who responds to this Enquiry.
    #[serde(rename = "ResponderParty")]
    pub responder_party: cac::Party,
/// References to relevant documents for the enquiry such as the Contract folder or the lot in the
/// eTendering.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// Attachment that includes file-based enquiry.
    #[serde(default, rename = "Attachment")]
    pub attachment: Vec<cac::Attachment>,
}
