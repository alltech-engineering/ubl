#[derive(Debug, Deserialize, Serialize)]
/// A document sent from a Transportation Network Manager to a Transport Service Provider giving the
/// status of the whereabouts and schedule of the transport means involved in a transport service.
///
/// UBL Dictionary Entry Name: `Transport Progress Status. Details`
///
/// Generated from XSD type `TransportProgressStatusType`.
pub struct TransportProgressStatus {
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
/// (Deprecated) Indicates whether this document is a copy (true) or not (false).
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(rename = "IssueTime")]
    pub issue_time: udt::DateTime,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Indicates whether transport progress information is available.
    #[serde(default, rename = "StatusAvailableIndicator")]
    pub status_available_indicator: Option<udt::Indicator>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who sends the Transport Progress Status.
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
/// The Party who receives the Transport Progress Status.
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
/// The Party who issues the Transport Progress Status.
    #[serde(default, rename = "SourceIssuerParty")]
    pub source_issuer_party: Option<cac::Party>,
/// A reference to the Transport Progress Status Request document to which this status report is a
/// response.
    #[serde(default, rename = "TransportProgressStatusRequestDocumentReference")]
    pub transport_progress_status_request_document_reference:
        Option<cac::DocumentReference>,
/// The transport means by which the current transport service is effectuated.
    #[serde(rename = "TransportMeans")]
    pub transport_means: cac::TransportMeans,
/// Describes the status and schedule of the transport means operating the transport service as well as
/// the current location of the transport means.
    #[serde(default, rename = "TransportSchedule")]
    pub transport_schedule: Vec<cac::TransportSchedule>,
}
