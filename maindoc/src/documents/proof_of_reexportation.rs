#[derive(Debug, Deserialize, Serialize)]
pub struct ProofOfReexportation {
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
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
    #[serde(default, rename = "ExportingCustomsParty")]
    pub exporting_customs_party: Option<cac::Party>,
    #[serde(default, rename = "ImportingGuarantorParty")]
    pub importing_guarantor_party: Option<cac::Party>,
    #[serde(default, rename = "ExportingGuarantorParty")]
    pub exporting_guarantor_party: Option<cac::Party>,
    #[serde(default, rename = "GoodsItemPassportCounterfoil")]
    pub goods_item_passport_counterfoil: Vec<cac::GoodsItemPassportCounterfoil>,
    #[serde(default, rename = "ReexportationEvidence")]
    pub reexportation_evidence: Vec<cac::Evidence>,
    #[serde(default, rename = "GoodsItemPassportAttachment")]
    pub goods_item_passport_attachment: Option<cac::Attachment>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
