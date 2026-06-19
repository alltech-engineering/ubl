#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingCriterionPropertyGroup {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "PropertyGroupTypeCode")]
    pub property_group_type_code: Option<cct::Code>,
    #[serde(default, rename = "FulfilmentIndicator")]
    pub fulfilment_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "FulfilmentIndicatorTypeCode")]
    pub fulfilment_indicator_type_code: Option<cct::Code>,
    #[serde(default, rename = "TenderingCriterionProperty")]
    pub tendering_criterion_property: Vec<TenderingCriterionProperty>,
    #[serde(default, rename = "SubsidiaryTenderingCriterionPropertyGroup")]
    pub subsidiary_tendering_criterion_property_group: Vec<TenderingCriterionPropertyGroup>,
}
