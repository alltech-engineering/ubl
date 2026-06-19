#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastException {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "ForecastPurposeCode")]
    pub forecast_purpose_code: cct::Code,
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: cct::Code,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(rename = "DataSourceCode")]
    pub data_source_code: cct::Code,
    #[serde(default, rename = "ComparisonDataCode")]
    pub comparison_data_code: Option<cct::Code>,
    #[serde(default, rename = "ComparisonForecastIssueTime")]
    pub comparison_forecast_issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "ComparisonForecastIssueDate")]
    pub comparison_forecast_issue_date: Option<udt::DateTime>,
}
