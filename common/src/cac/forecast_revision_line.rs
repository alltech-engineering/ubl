#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastRevisionLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(rename = "RevisedForecastLineID")]
    pub revised_forecast_line_id: super::cct::IdentifierType,
    #[serde(rename = "SourceForecastIssueDate")]
    pub source_forecast_issue_date: super::udt::DateTimeType,
    #[serde(rename = "SourceForecastIssueTime")]
    pub source_forecast_issue_time: super::udt::DateTimeType,
    #[serde(default, rename = "AdjustmentReasonCode")]
    pub adjustment_reason_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ForecastPeriod")]
    pub forecast_period: Option<Period>,
    #[serde(default, rename = "SalesItem")]
    pub sales_item: Option<SalesItem>,
}
