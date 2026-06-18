#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionAverage {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "AverageAmount")]
    pub average_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
}
