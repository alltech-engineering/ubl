#[derive(Debug, Deserialize, Serialize)]
pub struct ExceptionCriteriaLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(rename = "ThresholdValueComparisonCode")]
    pub threshold_value_comparison_code: super::cct::CodeType,
    #[serde(rename = "ThresholdQuantity")]
    pub threshold_quantity: super::cct::QuantityType,
    #[serde(default, rename = "ExceptionStatusCode")]
    pub exception_status_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CollaborationPriorityCode")]
    pub collaboration_priority_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ExceptionResolutionCode")]
    pub exception_resolution_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "EffectivePeriod")]
    pub effective_period: Option<Period>,
    #[serde(default, rename = "SupplyItem")]
    pub supply_item: Vec<Item>,
    #[serde(default, rename = "ForecastExceptionCriterionLine")]
    pub forecast_exception_criterion_line:
        Option<ForecastExceptionCriterionLine>,
}
