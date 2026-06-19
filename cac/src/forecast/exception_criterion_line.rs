#[derive(Debug, Deserialize, Serialize)]
/// Establishes the criterion for one of the three types of exceptions. This class provides criteria for
/// the kind of forecast exception, the identification of the purpose of the forecast, the source of
/// data and the time basis criterion for the exception.
///
/// UBL Dictionary Entry Name: `Forecast Exception Criterion Line. Details`
///
/// Generated from XSD type `ForecastExceptionCriterionLineType`.
pub struct ForecastExceptionCriterionLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A description of the purpose for the forecast that is assigned to each forecast data item exception
/// criterion.
    #[serde(rename = "ForecastPurposeCode")]
    pub forecast_purpose_code: cct::Code,
/// A description of a Forecast selected from a list.
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: cct::Code,
/// If it is a forecast comparison exception, this value indicates the other source of information.
    #[serde(default, rename = "ComparisonDataSourceCode")]
    pub comparison_data_source_code: Option<cct::Code>,
/// Indication of the partner who provides the information.
    #[serde(rename = "DataSourceCode")]
    pub data_source_code: cct::Code,
/// Time basis in days for the Exception.
    #[serde(default, rename = "TimeDeltaDaysQuantity")]
    pub time_delta_days_quantity: Option<cct::Quantity>,
}
