#[derive(Debug, Deserialize, Serialize)]
/// A class to define a measurable dimension (length, mass, weight, volume, or area) of an item.
///
/// UBL Dictionary Entry Name: `Dimension. Details`
///
/// Generated from XSD type `DimensionType`.
pub struct Dimension {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the attribute to which the measure applies.
    #[serde(rename = "AttributeID")]
    pub attribute_id: cct::Identifier,
/// The measurement value.
    #[serde(default, rename = "Measure")]
    pub measure: Option<cct::Measure>,
/// Text describing the measurement attribute.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The minimum value in a range of measurement of this dimension.
    #[serde(default, rename = "MinimumMeasure")]
    pub minimum_measure: Option<cct::Measure>,
/// The maximum value in a range of measurement of this dimension.
    #[serde(default, rename = "MaximumMeasure")]
    pub maximum_measure: Option<cct::Measure>,
}
