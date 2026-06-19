#[derive(Debug, Deserialize, Serialize)]
/// A customs declaration document for exporting goods
///
/// UBL Dictionary Entry Name: `Export Customs Declaration. Details`
///
/// Generated from XSD type `ExportCustomsDeclarationType`.
pub struct ExportCustomsDeclaration {
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
/// A code specifying the type of export
    #[serde(default, rename = "ExportTypeCode")]
    pub export_type_code: Option<cct::Code>,
/// A code specifying the reason for the goods being exported
    #[serde(default, rename = "ExportReasonCode")]
    pub export_reason_code: Option<cct::Code>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Identifies the current version of this export customs declaration
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// The Party who exports the goods or has similar right of disposal over them at the time of export.
    #[serde(rename = "ExporterParty")]
    pub exporter_party: cac::Party,
/// The reference to the customs declaration of the goods being exported.
    #[serde(rename = "CustomsDeclaration")]
    pub customs_declaration: cac::CustomsDeclaration,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
