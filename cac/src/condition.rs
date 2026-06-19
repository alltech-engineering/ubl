#[derive(Debug, Deserialize, Serialize)]
/// A class to define a measurable condition of an object.
///
/// UBL Dictionary Entry Name: `Condition. Details`
///
/// Generated from XSD type `ConditionType`.
pub struct Condition {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the attribute that applies to the condition.
    #[serde(rename = "AttributeID")]
    pub attribute_id: cct::Identifier,
/// The measurement value.
    #[serde(default, rename = "Measure")]
    pub measure: Option<cct::Measure>,
/// Text describing the attribute that applies to the condition.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The minimum value in a range of measurement for this condition.
    #[serde(default, rename = "MinimumMeasure")]
    pub minimum_measure: Option<cct::Measure>,
/// The maximum value in a range of measurement for this condition.
    #[serde(default, rename = "MaximumMeasure")]
    pub maximum_measure: Option<cct::Measure>,
}
