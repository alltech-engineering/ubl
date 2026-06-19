#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingCriterion {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "CriterionTypeCode")]
    pub criterion_type_code: Option<cct::Code>,
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "WeightNumeric")]
    pub weight_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "FulfilmentIndicator")]
    pub fulfilment_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "FulfilmentIndicatorTypeCode")]
    pub fulfilment_indicator_type_code: Option<cct::Code>,
    #[serde(default, rename = "EvaluationMethodTypeCode")]
    pub evaluation_method_type_code: Option<cct::Code>,
    #[serde(default, rename = "WeightingConsiderationDescription")]
    pub weighting_consideration_description: Vec<cct::Text>,
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: Vec<crate::ProcurementProjectLotReference>,
    #[serde(default, rename = "CommodityClassification")]
    pub commodity_classification: Vec<crate::CommodityClassification>,
    #[serde(default, rename = "SubTenderingCriterion")]
    pub sub_tendering_criterion: Vec<TenderingCriterion>,
    #[serde(default, rename = "Legislation")]
    pub legislation: Vec<crate::Legislation>,
    #[serde(default, rename = "TenderingCriterionPropertyGroup")]
    pub tendering_criterion_property_group: Vec<TenderingCriterionPropertyGroup>,
}
