#[derive(Debug, Deserialize, Serialize)]
pub struct ExceptionNotificationLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "ExceptionStatusCode")]
    pub exception_status_code: Option<cct::Code>,
    #[serde(default, rename = "CollaborationPriorityCode")]
    pub collaboration_priority_code: Option<cct::Code>,
    #[serde(default, rename = "ResolutionCode")]
    pub resolution_code: Option<cct::Code>,
    #[serde(rename = "ComparedValueMeasure")]
    pub compared_value_measure: cct::Measure,
    #[serde(rename = "SourceValueMeasure")]
    pub source_value_measure: cct::Measure,
    #[serde(default, rename = "VarianceQuantity")]
    pub variance_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: Option<cct::Code>,
    #[serde(default, rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: Option<cct::Code>,
    #[serde(default, rename = "ExceptionObservationPeriod")]
    pub exception_observation_period: Option<crate::Period>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "ForecastException")]
    pub forecast_exception: Option<crate::ForecastException>,
    #[serde(rename = "SupplyItem")]
    pub supply_item: crate::Item,
}
