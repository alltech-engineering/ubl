#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingCriterion {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CriterionTypeCode")]
    pub criterion_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Name")]
    pub name: Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "WeightNumeric")]
    pub weight_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "FulfilmentIndicator")]
    pub fulfilment_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "FulfilmentIndicatorTypeCode")]
    pub fulfilment_indicator_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "EvaluationMethodTypeCode")]
    pub evaluation_method_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "WeightingConsiderationDescription")]
    pub weighting_consideration_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: Vec<ProcurementProjectLotReference>,
    #[serde(default, rename = "CommodityClassification")]
    pub commodity_classification: Vec<CommodityClassification>,
    #[serde(default, rename = "SubTenderingCriterion")]
    pub sub_tendering_criterion: Vec<TenderingCriterion>,
    #[serde(default, rename = "Legislation")]
    pub legislation: Vec<Legislation>,
    #[serde(default, rename = "TenderingCriterionPropertyGroup")]
    pub tendering_criterion_property_group: Vec<TenderingCriterionPropertyGroup>,
}
