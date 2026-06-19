#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a specific property of an item.
///
/// UBL Dictionary Entry Name: `Item Property. Details`
///
/// Generated from XSD type `ItemPropertyType`.
pub struct ItemProperty {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this property of an item.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The name of this item property.
    #[serde(rename = "Name")]
    pub name: cct::Text,
/// The name of this item property, expressed as a code.
    #[serde(default, rename = "NameCode")]
    pub name_code: Option<cct::Code>,
/// The method of testing the value of this item property.
    #[serde(default, rename = "TestMethod")]
    pub test_method: Option<cct::Text>,
/// The value of this item property, expressed as text.
    #[serde(default, rename = "Value")]
    pub value: Option<cct::Text>,
/// The value of this item property, expressed as a quantity.
    #[serde(default, rename = "ValueQuantity")]
    pub value_quantity: Option<cct::Quantity>,
/// Text qualifying the value of the property.
    #[serde(default, rename = "ValueQualifier")]
    pub value_qualifier: Vec<cct::Text>,
/// A code signifying the importance of this property in using it to describe a related Item.
    #[serde(default, rename = "ImportanceCode")]
    pub importance_code: Option<cct::Code>,
/// The value expressed as a text in case the property is a value in a list. For example, a colour.
    #[serde(default, rename = "ListValue")]
    pub list_value: Vec<cct::Text>,
/// The period during which this item property is valid.
    #[serde(default, rename = "UsabilityPeriod")]
    pub usability_period: Option<crate::Period>,
/// A description of the property group to which this item property belongs.
    #[serde(default, rename = "ItemPropertyGroup")]
    pub item_property_group: Vec<ItemPropertyGroup>,
/// The range of values for the dimensions of this property.
    #[serde(default, rename = "RangeDimension")]
    pub range_dimension: Option<crate::Dimension>,
/// A range of values for this item property.
    #[serde(default, rename = "ItemPropertyRange")]
    pub item_property_range: Option<ItemPropertyRange>,
/// Identifying information for this property, assigned according to a standard system.
    #[serde(default, rename = "StandardPropertyIdentification")]
    pub standard_property_identification: Option<crate::PropertyIdentification>,
/// A property subsidiary to this property.
    #[serde(default, rename = "SubItemProperty")]
    pub sub_item_property: Vec<ItemProperty>,
}
