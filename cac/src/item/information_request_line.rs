#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in an Item Information Request asking a trading partner for item
/// information.
///
/// UBL Dictionary Entry Name: `Item Information Request Line. Details`
///
/// Generated from XSD type `ItemInformationRequestLineType`.
pub struct ItemInformationRequestLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code signifying the frequency with which item information will be sent to the requester.
    #[serde(default, rename = "TimeFrequencyCode")]
    pub time_frequency_code: Option<cct::Code>,
/// A code used to identify the type of supply chain activity about which information request is issued.
/// Examples: CANCELED_ORDERS EMERGENCY_ORDERS ON_HAND ORDERS
    #[serde(default, rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: Option<cct::Code>,
/// The information request can be either about supply chain activity or about forecasts or about
/// performance metrics, so it will be optional
    #[serde(default, rename = "ForecastTypeCode")]
    pub forecast_type_code: Option<cct::Code>,
/// A code signifying a measure of performance.
    #[serde(default, rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: Option<cct::Code>,
/// A period for which this information is requested.
    #[serde(default, rename = "Period")]
    pub period: Vec<crate::Period>,
/// Sales information for the item to which this line applies.
    #[serde(default, rename = "SalesItem")]
    pub sales_item: Vec<crate::SalesItem>,
}
