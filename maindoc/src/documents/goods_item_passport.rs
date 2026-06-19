#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsItemPassport {
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
    #[serde(default, rename = "StatusCode")]
    pub status_code: Option<cct::Code>,
    #[serde(default, rename = "Status")]
    pub status: Vec<cct::Text>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
    #[serde(default, rename = "ExportReasonCode")]
    pub export_reason_code: Option<cct::Code>,
    #[serde(default, rename = "ExportReason")]
    pub export_reason: Vec<cct::Text>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<cac::Period>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<cac::Party>,
    #[serde(rename = "HolderParty")]
    pub holder_party: cac::Party,
    #[serde(default, rename = "RepresentativeParty")]
    pub representative_party: Option<cac::Party>,
    #[serde(default, rename = "ExportingGuarantorParty")]
    pub exporting_guarantor_party: Option<cac::Party>,
    #[serde(default, rename = "ImportingGuarantorParty")]
    pub importing_guarantor_party: Option<cac::Party>,
    #[serde(default, rename = "ExportingCustomsParty")]
    pub exporting_customs_party: Option<cac::Party>,
    #[serde(default, rename = "ImportingCustomsParty")]
    pub importing_customs_party: Option<cac::Party>,
    #[serde(rename = "Shipment")]
    pub shipment: cac::Shipment,
    #[serde(default, rename = "GoodsItemPassportCounterfoil")]
    pub goods_item_passport_counterfoil: Vec<cac::GoodsItemPassportCounterfoil>,
    #[serde(default, rename = "IssuerEndorsement")]
    pub issuer_endorsement: Option<cac::Endorsement>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: Vec<cac::DocumentDistribution>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
