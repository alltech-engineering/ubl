#[derive(Debug, Deserialize, Serialize)]
pub struct CommonTransportationReport {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: Option<cct::Code>,
    #[serde(default, rename = "ReportTypeCode")]
    pub report_type_code: Option<cct::Code>,
    #[serde(default, rename = "ReportType")]
    pub report_type: Vec<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
    #[serde(rename = "ReporterParty")]
    pub reporter_party: cac::Party,
    #[serde(default, rename = "AuthorityParty")]
    pub authority_party: Option<cac::Party>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
    #[serde(default, rename = "ReportingLocation")]
    pub reporting_location: Option<cac::Location>,
    #[serde(default, rename = "Shipment")]
    pub shipment: Option<cac::Shipment>,
    #[serde(default, rename = "TransportMeans")]
    pub transport_means: Vec<cac::TransportMeans>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
