#[derive(Debug, Deserialize, Serialize)]
pub struct ItemProperty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(rename = "Name")]
    pub name: cct::Text,
    #[serde(default, rename = "NameCode")]
    pub name_code: Option<cct::Code>,
    #[serde(default, rename = "TestMethod")]
    pub test_method: Option<cct::Text>,
    #[serde(default, rename = "Value")]
    pub value: Option<cct::Text>,
    #[serde(default, rename = "ValueQuantity")]
    pub value_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "ValueQualifier")]
    pub value_qualifier: Vec<cct::Text>,
    #[serde(default, rename = "ImportanceCode")]
    pub importance_code: Option<cct::Code>,
    #[serde(default, rename = "ListValue")]
    pub list_value: Vec<cct::Text>,
    #[serde(default, rename = "UsabilityPeriod")]
    pub usability_period: Option<crate::Period>,
    #[serde(default, rename = "ItemPropertyGroup")]
    pub item_property_group: Vec<ItemPropertyGroup>,
    #[serde(default, rename = "RangeDimension")]
    pub range_dimension: Option<crate::Dimension>,
    #[serde(default, rename = "ItemPropertyRange")]
    pub item_property_range: Option<ItemPropertyRange>,
    #[serde(default, rename = "StandardPropertyIdentification")]
    pub standard_property_identification: Option<crate::PropertyIdentification>,
    #[serde(default, rename = "SubItemProperty")]
    pub sub_item_property: Vec<ItemProperty>,
}
