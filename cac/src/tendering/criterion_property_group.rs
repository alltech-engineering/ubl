#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a group of tendering criteria
///
/// UBL Dictionary Entry Name: `Tendering Criterion Property Group. Details`
///
/// Generated from XSD type `TenderingCriterionPropertyGroupType`.
pub struct TenderingCriterionPropertyGroup {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the group of criteria.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The name of the group.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// The textual description for this group.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A code signifying the type of the property group
    #[serde(default, rename = "PropertyGroupTypeCode")]
    pub property_group_type_code: Option<cct::Code>,
/// An indication that this group of criteria have been fulfilled.
    #[serde(default, rename = "FulfilmentIndicator")]
    pub fulfilment_indicator: Option<udt::Indicator>,
/// A code signifying how this group of criteria have been fulfilled.
    #[serde(default, rename = "FulfilmentIndicatorTypeCode")]
    pub fulfilment_indicator_type_code: Option<cct::Code>,
/// All the criteria properties comprising the tendering criterion.
    #[serde(default, rename = "TenderingCriterionProperty")]
    pub tendering_criterion_property: Vec<TenderingCriterionProperty>,
/// Subsidiary tendering criteria groups comprising this tendering criterion.
    #[serde(default, rename = "SubsidiaryTenderingCriterionPropertyGroup")]
    pub subsidiary_tendering_criterion_property_group: Vec<TenderingCriterionPropertyGroup>,
}
