#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingCriterionResponse {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "ValidatedCriterionPropertyID")]
    pub validated_criterion_property_id: Option<cct::Identifier>,
    #[serde(default, rename = "ConfidentialityLevelCode")]
    pub confidentiality_level_code: Option<cct::Code>,
    #[serde(default, rename = "ResponseValue")]
    pub response_value: Vec<crate::ResponseValue>,
    #[serde(default, rename = "ApplicablePeriod")]
    pub applicable_period: Vec<crate::Period>,
    #[serde(default, rename = "EvidenceSupplied")]
    pub evidence_supplied: Vec<crate::EvidenceSupplied>,
    #[serde(default, rename = "SuppliedEvidence")]
    pub supplied_evidence: Vec<crate::Evidence>,
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: Vec<crate::ProcurementProjectLotReference>,
    #[serde(default, rename = "CommodityClassification")]
    pub commodity_classification: Vec<crate::CommodityClassification>,
}
