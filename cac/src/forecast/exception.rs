#[derive(Debug, Deserialize, Serialize)]
/// As explained in Exception Criteria Line: Three types of exception criteria can be defined,
/// Operational, Metric or Forecast Exceptions. This class provides criteria for forecast exception
/// type: the identification of the purpose of the forecast, the source of data and the time basis
/// criteria for the exception.
///
/// UBL Dictionary Entry Name: `Forecast Exception. Details`
///
/// Generated from XSD type `ForecastExceptionType`.
pub struct ForecastException {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// It is either Sales forecast or Order Forecast. Definition can be changed like: "The purpose of the
/// Forecast (either sales or order), about which an exception criteria is being defined".
    #[serde(rename = "ForecastPurposeCode")]
    pub forecast_purpose_code: cct::Code,
/// A code signifying the type of forecast. Example of values are:BASE PROMOTIONAL SEASONAL TOTAL
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: cct::Code,
/// The date on which the forecast was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time at which the forecast was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// A code signifying the partner who provides this information.
    #[serde(rename = "DataSourceCode")]
    pub data_source_code: cct::Code,
/// A code signifying the partner providing the information in this forecast exception.
    #[serde(default, rename = "ComparisonDataCode")]
    pub comparison_data_code: Option<cct::Code>,
/// The time at which this comparison forecast was issued.
    #[serde(default, rename = "ComparisonForecastIssueTime")]
    pub comparison_forecast_issue_time: Option<udt::DateTime>,
/// The date on which this comparison forecast was issued.
    #[serde(default, rename = "ComparisonForecastIssueDate")]
    pub comparison_forecast_issue_date: Option<udt::DateTime>,
}
