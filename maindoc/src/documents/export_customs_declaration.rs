#[derive(Debug, Deserialize, Serialize)]
pub struct ExportCustomsDeclaration {
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
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "ExportTypeCode")]
    pub export_type_code: Option<cct::Code>,
    #[serde(default, rename = "ExportReasonCode")]
    pub export_reason_code: Option<cct::Code>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
    #[serde(rename = "ExporterParty")]
    pub exporter_party: cac::Party,
    #[serde(rename = "CustomsDeclaration")]
    pub customs_declaration: cac::CustomsDeclaration,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
