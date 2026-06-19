#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an item of criterion support for representations of capabilities or the ability
/// to meet tendering requirements, which an economic operator must provide for acceptance into a
/// tendering process.
///
/// UBL Dictionary Entry Name: `Tendering Criterion. Details`
///
/// Generated from XSD type `TenderingCriterionType`.
pub struct TenderingCriterion {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this item of criterion support.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A code signifying the type of criterion.
    #[serde(default, rename = "CriterionTypeCode")]
    pub criterion_type_code: Option<cct::Code>,
/// The name of the criterion.
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
/// The textual description for this criterion.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A weighting to provide for automatic scoring of the criterion.
    #[serde(default, rename = "WeightNumeric")]
    pub weight_numeric: Option<cct::Numeric>,
/// An indication that this criterion has been fulfilled.
    #[serde(default, rename = "FulfilmentIndicator")]
    pub fulfilment_indicator: Option<udt::Indicator>,
/// A code signifying how this criterion has been fulfilled.
    #[serde(default, rename = "FulfilmentIndicatorTypeCode")]
    pub fulfilment_indicator_type_code: Option<cct::Code>,
/// A code signifying the type of Evaluation.
    #[serde(default, rename = "EvaluationMethodTypeCode")]
    pub evaluation_method_type_code: Option<cct::Code>,
/// The textual description of the Weighting Description
    #[serde(default, rename = "WeightingConsiderationDescription")]
    pub weighting_consideration_description: Vec<cct::Text>,
/// One or more lots to which the tendering criterion applies
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: Vec<crate::ProcurementProjectLotReference>,
/// One or more classification to which this criterion applies
    #[serde(default, rename = "CommodityClassification")]
    pub commodity_classification: Vec<crate::CommodityClassification>,
/// One or more tendering subcriteria.
    #[serde(default, rename = "SubTenderingCriterion")]
    pub sub_tendering_criterion: Vec<TenderingCriterion>,
/// The legislation reference for the criterion.
    #[serde(default, rename = "Legislation")]
    pub legislation: Vec<crate::Legislation>,
/// The sets of properties that can be used to fulfil the tendering criterion.
    #[serde(default, rename = "TenderingCriterionPropertyGroup")]
    pub tendering_criterion_property_group: Vec<TenderingCriterionPropertyGroup>,
}
