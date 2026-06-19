#[derive(Debug, Deserialize, Serialize)]
pub struct ProofOfReexportationRequest {
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
    #[serde(rename = "GoodsItemPassportID")]
    pub goods_item_passport_id: cct::Identifier,
    #[serde(default, rename = "GoodsItemPassportCounterfoilID")]
    pub goods_item_passport_counterfoil_id: Option<cct::Identifier>,
    #[serde(rename = "ImportingGuarantorParty")]
    pub importing_guarantor_party: cac::Party,
    #[serde(rename = "ExportingGuarantorParty")]
    pub exporting_guarantor_party: cac::Party,
    #[serde(default, rename = "ImportingCustomsParty")]
    pub importing_customs_party: Option<cac::Party>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
