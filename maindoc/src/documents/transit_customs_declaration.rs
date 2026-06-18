#[derive(Debug, Deserialize, Serialize)]
pub struct TransitCustomsDeclaration {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<cct::CodeType>,
    #[serde(default, rename = "SubTypeCode")]
    pub sub_type_code: Option<cct::CodeType>,
    #[serde(default, rename = "NatureOfTransactionCode")]
    pub nature_of_transaction_code: Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<cac::Period>,
    #[serde(default, rename = "ExportCustomsExitOfficeLocation")]
    pub export_customs_exit_office_location: Option<cac::Location>,
    #[serde(default, rename = "TransitCustomsExitOfficeLocation")]
    pub transit_customs_exit_office_location: Option<cac::Location>,
    #[serde(default, rename = "ImportCustomsExitOfficeLocation")]
    pub import_customs_exit_office_location: Option<cac::Location>,
    #[serde(default, rename = "JurisdictionRegionAddress")]
    pub jurisdiction_region_address: Option<cac::Address>,
    #[serde(default, rename = "TransitExporterParty")]
    pub transit_exporter_party: Option<cac::Party>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: Option<cac::Party>,
    #[serde(default, rename = "ConsigneeParty")]
    pub consignee_party: Option<cac::Party>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: Option<cac::Party>,
    #[serde(rename = "CustomsParty")]
    pub customs_party: cac::Party,
    #[serde(default, rename = "NotifierParty")]
    pub notifier_party: Option<cac::Party>,
    #[serde(default, rename = "Shipment")]
    pub shipment: Vec<cac::Shipment>,
    #[serde(default, rename = "PreviousCustomsDeclaration")]
    pub previous_customs_declaration: Vec<cac::CustomsDeclaration>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
