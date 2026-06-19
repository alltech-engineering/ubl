#[derive(Debug, Deserialize, Serialize)]
/// A common document used for reporting transport related issues to authorities or regulators.
///
/// UBL Dictionary Entry Name: `Common Transportation Report. Details`
///
/// Generated from XSD type `CommonTransportationReportType`.
pub struct CommonTransportationReport {
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
/// An identifier for this document.
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
/// A code signifying the status of this Common Transportation Report with respect to its original
/// state.
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: Option<cct::Code>,
/// A code signifying the type of report being provided
    #[serde(default, rename = "ReportTypeCode")]
    pub report_type_code: Option<cct::Code>,
/// A text that identifies the type of report to business users.
    #[serde(default, rename = "ReportType")]
    pub report_type: Vec<cct::Text>,
/// Textual description of this document instance.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Identifies a version of a common transportation report in order to distinguish updates.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// The Party who provides this Common Transportation Report.
    #[serde(rename = "ReporterParty")]
    pub reporter_party: cac::Party,
/// The Party who receives the Common Transportation Report. This Party is normally an Authority or
/// regulator.
    #[serde(default, rename = "AuthorityParty")]
    pub authority_party: Option<cac::Party>,
/// The Party who sends this Report.
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
/// The Party who receives this Report.
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
/// A location to which this common transportation report applies.
    #[serde(default, rename = "ReportingLocation")]
    pub reporting_location: Option<cac::Location>,
/// A shipment to which this common transportation report applies.
    #[serde(default, rename = "Shipment")]
    pub shipment: Option<cac::Shipment>,
/// A means of transport used in relation to this common transportation report.
    #[serde(default, rename = "TransportMeans")]
    pub transport_means: Vec<cac::TransportMeans>,
/// A reference to a document relevant for or associated with this common transportation report.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
