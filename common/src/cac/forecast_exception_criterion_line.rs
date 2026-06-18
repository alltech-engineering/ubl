#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastExceptionCriterionLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ForecastPurposeCode")]
    pub forecast_purpose_code: super::cct::CodeType,
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: super::cct::CodeType,
    #[serde(default, rename = "ComparisonDataSourceCode")]
    pub comparison_data_source_code: Option<super::cct::CodeType>,
    #[serde(rename = "DataSourceCode")]
    pub data_source_code: super::cct::CodeType,
    #[serde(default, rename = "TimeDeltaDaysQuantity")]
    pub time_delta_days_quantity: Option<super::cct::QuantityType>,
}
