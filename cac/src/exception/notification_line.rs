#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in an Exception Notification.
///
/// UBL Dictionary Entry Name: `Exception Notification Line. Details`
///
/// Generated from XSD type `ExceptionNotificationLineType`.
pub struct ExceptionNotificationLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this exception notification line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Text describing the exception.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A code signifying status specific to a shipment exception.
    #[serde(default, rename = "ExceptionStatusCode")]
    pub exception_status_code: Option<cct::Code>,
/// Priority of Exception.
    #[serde(default, rename = "CollaborationPriorityCode")]
    pub collaboration_priority_code: Option<cct::Code>,
/// Coded representation of possible resolution methods". Possible values are:
/// DEFAULT_TO_AVERAGE_OF_COMPARED_VALUES DEFAULT_TO_BUYERS_VALUE DEFAULT_TO_HIGH_VALUE
/// DEFAULT_TO_LOW_VALUE DEFAULT_TO_SELLERS_VALUE MANUAL_RESOLUTION MUTUALLY_DEFINED
    #[serde(default, rename = "ResolutionCode")]
    pub resolution_code: Option<cct::Code>,
/// The value that was compared with the source value that resulted in the exception
    #[serde(rename = "ComparedValueMeasure")]
    pub compared_value_measure: cct::Measure,
/// The value used as the basis of comparison
    #[serde(rename = "SourceValueMeasure")]
    pub source_value_measure: cct::Measure,
/// The variance of a data item from an expected value during a particular time interval.
    #[serde(default, rename = "VarianceQuantity")]
    pub variance_quantity: Option<cct::Quantity>,
/// Establishes the criterion for one of the three types of exceptions: Operational, performance metric
/// and forecast. This reports an exception notification about an operational exception. Description
/// could be: A code used to identify an operational exception. Possible values are: CANCELED_ORDERS
/// EMERGENCY_ORDERS ON_HAND ORDERS RECEIPTS SALES SHIPMENTS
    #[serde(default, rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: Option<cct::Code>,
/// A code used to identify a measure of performance. It defines the type of the Performance Metric on
/// which an exception criteria is being defined
    #[serde(default, rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: Option<cct::Code>,
/// The period (start-end date) when this exception is observed
    #[serde(default, rename = "ExceptionObservationPeriod")]
    pub exception_observation_period: Option<crate::Period>,
/// A reference to Exception Criteria document can be provided.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
/// A forecast accuracy or comparison exception.
    #[serde(default, rename = "ForecastException")]
    pub forecast_exception: Option<crate::ForecastException>,
/// The product associated with this exception notification line.
    #[serde(rename = "SupplyItem")]
    pub supply_item: crate::Item,
}
