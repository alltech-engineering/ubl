#[derive(Debug, Deserialize, Serialize)]
pub struct Dimension {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "AttributeID")]
    pub attribute_id: super::cct::IdentifierType,
    #[serde(default, rename = "Measure")]
    pub measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "MinimumMeasure")]
    pub minimum_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "MaximumMeasure")]
    pub maximum_measure: Option<super::cct::MeasureType>,
}
