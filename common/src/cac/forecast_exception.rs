#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastException {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ForecastPurposeCode")]
    pub forecast_purpose_code: super::cct::CodeType,
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: super::cct::CodeType,
    #[serde(rename = "IssueDate")]
    pub issue_date: super::udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<super::udt::DateTimeType>,
    #[serde(rename = "DataSourceCode")]
    pub data_source_code: super::cct::CodeType,
    #[serde(default, rename = "ComparisonDataCode")]
    pub comparison_data_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ComparisonForecastIssueTime")]
    pub comparison_forecast_issue_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ComparisonForecastIssueDate")]
    pub comparison_forecast_issue_date: Option<super::udt::DateTimeType>,
}
