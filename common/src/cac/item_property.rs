#[derive(Debug, Deserialize, Serialize)]
pub struct ItemProperty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(rename = "Name")]
    pub name: super::cct::TextType,
    #[serde(default, rename = "NameCode")]
    pub name_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "TestMethod")]
    pub test_method: Option<super::cct::TextType>,
    #[serde(default, rename = "Value")]
    pub value: Option<super::cct::TextType>,
    #[serde(default, rename = "ValueQuantity")]
    pub value_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "ValueQualifier")]
    pub value_qualifier: Vec<super::cct::TextType>,
    #[serde(default, rename = "ImportanceCode")]
    pub importance_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ListValue")]
    pub list_value: Vec<super::cct::TextType>,
    #[serde(default, rename = "UsabilityPeriod")]
    pub usability_period: Option<Period>,
    #[serde(default, rename = "ItemPropertyGroup")]
    pub item_property_group: Vec<ItemPropertyGroup>,
    #[serde(default, rename = "RangeDimension")]
    pub range_dimension: Option<Dimension>,
    #[serde(default, rename = "ItemPropertyRange")]
    pub item_property_range: Option<ItemPropertyRange>,
    #[serde(default, rename = "StandardPropertyIdentification")]
    pub standard_property_identification: Option<PropertyIdentification>,
    #[serde(default, rename = "SubItemProperty")]
    pub sub_item_property: Vec<ItemProperty>,
}
