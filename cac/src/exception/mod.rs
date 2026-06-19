use serde::{Deserialize, Serialize};

include!("notification_line.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in an ExceptionCriteria document that specifies a threshold for forecast
/// variance, product activity, or performance history, the exceeding of which will trigger an exception
/// message.
///
/// UBL Dictionary Entry Name: `Exception Criteria Line. Details`
///
/// Generated from XSD type `ExceptionCriteriaLineType`.
pub struct ExceptionCriteriaLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this exception criteria line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Type of comparison to be carried out in reference to the set threshold." Allowed values are:
/// EXCEEDS_EXCEPTION_VALUE FALLS_BELOW_EXCEPTION_VALUE
    #[serde(rename = "ThresholdValueComparisonCode")]
    pub threshold_value_comparison_code: cct::Code,
/// A quantity beyond which an exception will be triggered.
    #[serde(rename = "ThresholdQuantity")]
    pub threshold_quantity: cct::Quantity,
/// A code signifying status specific to a shipment exception.
    #[serde(default, rename = "ExceptionStatusCode")]
    pub exception_status_code: Option<cct::Code>,
/// A collaboratively assigned code signifying priority of the Exception. Possible values are: HIGH,
/// LOW, MEDIUM
    #[serde(default, rename = "CollaborationPriorityCode")]
    pub collaboration_priority_code: Option<cct::Code>,
/// Coded representation of possible resolution methods". Possible values are:
/// DEFAULT_TO_AVERAGE_OF_COMPARED_VALUES DEFAULT_TO_BUYERS_VALUE DEFAULT_TO_HIGH_VALUE
/// DEFAULT_TO_LOW_VALUE DEFAULT_TO_SELLERS_VALUE MANUAL_RESOLUTION MUTUALLY_DEFINED
    #[serde(default, rename = "ExceptionResolutionCode")]
    pub exception_resolution_code: Option<cct::Code>,
/// Establishes the criterion for one of the three types of exceptions. There can be three types of
/// exception criteria: Operational, Metric and Forecast Exceptions. This will be set if this Exception
/// is about an Operational Exception. Description could be: A code used to identify an operational
/// exception. Possible values are: CANCELED_ORDERS EMERGENCY_ORDERS ON_HAND ORDERS RECEIPTS SALES
/// SHIPMENTS
    #[serde(default, rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: Option<cct::Code>,
/// A code signifying a measure of performance.
    #[serde(default, rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: Option<cct::Code>,
/// The period during which this exception criteria line is in effect.
    #[serde(default, rename = "EffectivePeriod")]
    pub effective_period: Option<crate::Period>,
/// The Trade Item that is the subject of the Exception Criterion.
    #[serde(default, rename = "SupplyItem")]
    pub supply_item: Vec<crate::Item>,
/// Establishes the criterion for one of the three types of exceptions. This class provides the
/// criterion for the kind of forecast exception, the identification of the purpose of the forecast, the
/// source of data and the time basis criterion for the exception.
    #[serde(default, rename = "ForecastExceptionCriterionLine")]
    pub forecast_exception_criterion_line: Option<crate::ForecastExceptionCriterionLine>,
}
