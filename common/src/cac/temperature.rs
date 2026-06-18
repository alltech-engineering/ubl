#[derive(Debug, Deserialize, Serialize)]
pub struct Temperature {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "AttributeID")]
    pub attribute_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Measure")]
    pub measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "MeasureCode")]
    pub measure_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
}
