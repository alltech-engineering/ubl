#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionHistory {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "MeterNumber")]
    pub meter_number: Option<super::cct::TextType>,
    #[serde(rename = "Quantity")]
    pub quantity: super::cct::QuantityType,
    #[serde(default, rename = "Amount")]
    pub amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "ConsumptionLevelCode")]
    pub consumption_level_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ConsumptionLevel")]
    pub consumption_level: Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(rename = "Period")]
    pub period: Period,
}
