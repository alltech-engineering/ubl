#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingCriterionPropertyGroup {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "PropertyGroupTypeCode")]
    pub property_group_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "FulfilmentIndicator")]
    pub fulfilment_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "FulfilmentIndicatorTypeCode")]
    pub fulfilment_indicator_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "TenderingCriterionProperty")]
    pub tendering_criterion_property: Vec<TenderingCriterionProperty>,
    #[serde(default, rename = "SubsidiaryTenderingCriterionPropertyGroup")]
    pub subsidiary_tendering_criterion_property_group:
        Vec<TenderingCriterionPropertyGroup>,
}
