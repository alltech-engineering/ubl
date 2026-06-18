#[derive(Debug, Deserialize, Serialize)]
pub struct RetailPlannedImpact {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "Amount")]
    pub amount: super::cct::AmountType,
    #[serde(rename = "ForecastPurposeCode")]
    pub forecast_purpose_code: super::cct::CodeType,
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: super::cct::CodeType,
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
}
