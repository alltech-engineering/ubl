#[derive(Debug, Deserialize, Serialize)]
pub struct ItemInformationRequestLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "TimeFrequencyCode")]
    pub time_frequency_code: Option<cct::Code>,
    #[serde(default, rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: Option<cct::Code>,
    #[serde(default, rename = "ForecastTypeCode")]
    pub forecast_type_code: Option<cct::Code>,
    #[serde(default, rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: Option<cct::Code>,
    #[serde(default, rename = "Period")]
    pub period: Vec<crate::Period>,
    #[serde(default, rename = "SalesItem")]
    pub sales_item: Vec<crate::SalesItem>,
}
