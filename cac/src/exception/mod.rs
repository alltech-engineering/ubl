use serde::{Deserialize, Serialize};

include!("notification_line.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct ExceptionCriteriaLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(rename = "ThresholdValueComparisonCode")]
    pub threshold_value_comparison_code: cct::Code,
    #[serde(rename = "ThresholdQuantity")]
    pub threshold_quantity: cct::Quantity,
    #[serde(default, rename = "ExceptionStatusCode")]
    pub exception_status_code: Option<cct::Code>,
    #[serde(default, rename = "CollaborationPriorityCode")]
    pub collaboration_priority_code: Option<cct::Code>,
    #[serde(default, rename = "ExceptionResolutionCode")]
    pub exception_resolution_code: Option<cct::Code>,
    #[serde(default, rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: Option<cct::Code>,
    #[serde(default, rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: Option<cct::Code>,
    #[serde(default, rename = "EffectivePeriod")]
    pub effective_period: Option<crate::Period>,
    #[serde(default, rename = "SupplyItem")]
    pub supply_item: Vec<crate::Item>,
    #[serde(default, rename = "ForecastExceptionCriterionLine")]
    pub forecast_exception_criterion_line: Option<crate::ForecastExceptionCriterionLine>,
}
