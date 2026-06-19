#[derive(Debug, Deserialize, Serialize)]
pub struct Temperature {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "AttributeID")]
    pub attribute_id: Option<cct::Identifier>,
    #[serde(default, rename = "Measure")]
    pub measure: Option<cct::Measure>,
    #[serde(default, rename = "MeasureCode")]
    pub measure_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
