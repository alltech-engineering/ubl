#[derive(Debug, Deserialize, Serialize)]
/// A document used to provide information about a business.
///
/// UBL Dictionary Entry Name: `Business Information. Details`
///
/// Generated from XSD type `BusinessInformationType`.
pub struct BusinessInformation {
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
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Identifies the current version of this Business Registration Information.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// Identifies the previous version of this Business Registration Information.
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: Option<cct::Identifier>,
/// Textual description of the document instance.
    #[serde(default, rename = "BriefDescription")]
    pub brief_description: Vec<cct::Text>,
/// The requested publication date for this Business Registration Information Notice.
    #[serde(default, rename = "RequestedPublicationDate")]
    pub requested_publication_date: Option<udt::DateTime>,
/// Information about the law that defines the regulatory domain.
    #[serde(default, rename = "RegulatoryDomain")]
    pub regulatory_domain: Vec<cct::Text>,
/// The type of notice.
    #[serde(default, rename = "NoticeTypeCode")]
    pub notice_type_code: Option<cct::Code>,
/// The language used for this notice.
    #[serde(default, rename = "NoticeLanguageCode")]
    pub notice_language_code: Option<cct::Code>,
/// An additional official language used in this document.
    #[serde(default, rename = "AdditionalNoticeLanguage")]
    pub additional_notice_language: Vec<cac::Language>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who sends this Business Registration Information. This Party may be the owner of this
/// Business Registration Information or a third-Party who acts on behalf of the owner (e.g. business
/// network).
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
/// The Party who receives this Business Registration Information.
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
/// The Party who owns this Business Registration Information.
    #[serde(rename = "BusinessParty")]
    pub business_party: cac::Party,
/// A reference to a company brochure document.
    #[serde(default, rename = "BrochureDocumentReference")]
    pub brochure_document_reference: Vec<cac::DocumentReference>,
/// A reference to an additional document (e.g. presentations).
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// The business capabilities of the party.
    #[serde(default, rename = "BusinessCapability")]
    pub business_capability: Vec<cac::Capability>,
/// A group of Business Parties.
    #[serde(default, rename = "BusinessPartyGroup")]
    pub business_party_group: Vec<cac::PartyGroup>,
/// The type of operation for which this document is created.
    #[serde(default, rename = "OperationType")]
    pub operation_type: Vec<cac::Operation>,
/// The subtype of this notice.
    #[serde(default, rename = "NoticeSubType")]
    pub notice_sub_type: Option<cac::NoticeSubCategory>,
}
