#[derive(Debug, Deserialize, Serialize)]
pub struct ImportCustomsDeclaration {
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
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<cct::Code>,
    #[serde(default, rename = "SubTypeCode")]
    pub sub_type_code: Option<cct::Code>,
    #[serde(default, rename = "NatureOfTransactionCode")]
    pub nature_of_transaction_code: Option<cct::Code>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<cac::Period>,
    #[serde(default, rename = "CustomsExitOfficeLocation")]
    pub customs_exit_office_location: Option<cac::Location>,
    #[serde(default, rename = "JurisdictionRegionAddress")]
    pub jurisdiction_region_address: Option<cac::Address>,
    #[serde(default, rename = "ImporterParty")]
    pub importer_party: Option<cac::Party>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: Option<cac::Party>,
    #[serde(default, rename = "ConsigneeParty")]
    pub consignee_party: Option<cac::Party>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: Option<cac::Party>,
    #[serde(default, rename = "CustomsParty")]
    pub customs_party: Option<cac::Party>,
    #[serde(default, rename = "NotifierParty")]
    pub notifier_party: Option<cac::Party>,
    #[serde(rename = "Shipment")]
    pub shipment: cac::Shipment,
    #[serde(default, rename = "PreviousCustomsDeclaration")]
    pub previous_customs_declaration: Option<cac::CustomsDeclaration>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
