#[derive(Debug, Deserialize, Serialize)]
pub struct Condition {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "AttributeID")]
    pub attribute_id: cct::Identifier,
    #[serde(default, rename = "Measure")]
    pub measure: Option<cct::Measure>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "MinimumMeasure")]
    pub minimum_measure: Option<cct::Measure>,
    #[serde(default, rename = "MaximumMeasure")]
    pub maximum_measure: Option<cct::Measure>,
}
