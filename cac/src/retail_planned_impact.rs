#[derive(Debug, Deserialize, Serialize)]
pub struct RetailPlannedImpact {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "Amount")]
    pub amount: cct::Amount,
    #[serde(rename = "ForecastPurposeCode")]
    pub forecast_purpose_code: cct::Code,
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: cct::Code,
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
}
