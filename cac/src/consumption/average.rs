#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionAverage {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "AverageAmount")]
    pub average_amount: Option<cct::Amount>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
