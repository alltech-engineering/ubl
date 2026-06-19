#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionHistory {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "MeterNumber")]
    pub meter_number: Option<cct::Text>,
    #[serde(rename = "Quantity")]
    pub quantity: cct::Quantity,
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
    #[serde(default, rename = "ConsumptionLevelCode")]
    pub consumption_level_code: Option<cct::Code>,
    #[serde(default, rename = "ConsumptionLevel")]
    pub consumption_level: Option<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(rename = "Period")]
    pub period: crate::Period,
}
