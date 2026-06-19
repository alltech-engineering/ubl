#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastRevisionLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(rename = "RevisedForecastLineID")]
    pub revised_forecast_line_id: cct::Identifier,
    #[serde(rename = "SourceForecastIssueDate")]
    pub source_forecast_issue_date: udt::DateTime,
    #[serde(rename = "SourceForecastIssueTime")]
    pub source_forecast_issue_time: udt::DateTime,
    #[serde(default, rename = "AdjustmentReasonCode")]
    pub adjustment_reason_code: Option<cct::Code>,
    #[serde(default, rename = "ForecastPeriod")]
    pub forecast_period: Option<crate::Period>,
    #[serde(default, rename = "SalesItem")]
    pub sales_item: Option<crate::SalesItem>,
}
