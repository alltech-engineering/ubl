#[derive(Debug, Deserialize, Serialize)]
pub struct ItemInformationRequestLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "TimeFrequencyCode")]
    pub time_frequency_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ForecastTypeCode")]
    pub forecast_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Period")]
    pub period: Vec<Period>,
    #[serde(default, rename = "SalesItem")]
    pub sales_item: Vec<SalesItem>,
}
