#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a range of values for an item property.
///
/// UBL Dictionary Entry Name: `Item Property Range. Details`
///
/// Generated from XSD type `ItemPropertyRangeType`.
pub struct ItemPropertyRange {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// The minimum value in this range of values.
    #[serde(default, rename = "MinimumValue")]
    pub minimum_value: Option<cct::Text>,
/// The maximum value in this range of values.
    #[serde(default, rename = "MaximumValue")]
    pub maximum_value: Option<cct::Text>,
}
