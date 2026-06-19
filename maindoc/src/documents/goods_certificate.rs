#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsCertificate {
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
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<cac::Period>,
    #[serde(default, rename = "ApplicableTerritoryAddress")]
    pub applicable_territory_address: Option<cac::Address>,
    #[serde(default, rename = "ExporterParty")]
    pub exporter_party: Option<cac::Party>,
    #[serde(default, rename = "ImporterParty")]
    pub importer_party: Option<cac::Party>,
    #[serde(default, rename = "WarehouseParty")]
    pub warehouse_party: Option<cac::Party>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: Option<cac::Party>,
    #[serde(default, rename = "ConsigneeParty")]
    pub consignee_party: Option<cac::Party>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: Option<cac::Party>,
    #[serde(rename = "IssuerParty")]
    pub issuer_party: cac::Party,
    #[serde(default, rename = "LegalAuthorityParty")]
    pub legal_authority_party: Option<cac::Party>,
    #[serde(default, rename = "ApplicantParty")]
    pub applicant_party: Option<cac::Party>,
    #[serde(rename = "Shipment")]
    pub shipment: cac::Shipment,
    #[serde(default, rename = "Attestation")]
    pub attestation: Vec<cac::Attestation>,
    #[serde(default, rename = "GoodsProcessing")]
    pub goods_processing: Vec<cac::GoodsProcessing>,
    #[serde(default, rename = "OriginalDocumentReference")]
    pub original_document_reference: Option<cac::DocumentReference>,
    #[serde(default, rename = "PreviousDocumentReference")]
    pub previous_document_reference: Option<cac::DocumentReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
