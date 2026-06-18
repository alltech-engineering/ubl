#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingCriterionResponse {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "ValidatedCriterionPropertyID")]
    pub validated_criterion_property_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ConfidentialityLevelCode")]
    pub confidentiality_level_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ResponseValue")]
    pub response_value: Vec<ResponseValue>,
    #[serde(default, rename = "ApplicablePeriod")]
    pub applicable_period: Vec<Period>,
    #[serde(default, rename = "EvidenceSupplied")]
    pub evidence_supplied: Vec<EvidenceSupplied>,
    #[serde(default, rename = "SuppliedEvidence")]
    pub supplied_evidence: Vec<Evidence>,
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: Vec<ProcurementProjectLotReference>,
    #[serde(default, rename = "CommodityClassification")]
    pub commodity_classification: Vec<CommodityClassification>,
}
