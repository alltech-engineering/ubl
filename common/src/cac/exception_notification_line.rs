#[derive(Debug, Deserialize, Serialize)]
pub struct ExceptionNotificationLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "ExceptionStatusCode")]
    pub exception_status_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CollaborationPriorityCode")]
    pub collaboration_priority_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ResolutionCode")]
    pub resolution_code: Option<super::cct::CodeType>,
    #[serde(rename = "ComparedValueMeasure")]
    pub compared_value_measure: super::cct::MeasureType,
    #[serde(rename = "SourceValueMeasure")]
    pub source_value_measure: super::cct::MeasureType,
    #[serde(default, rename = "VarianceQuantity")]
    pub variance_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ExceptionObservationPeriod")]
    pub exception_observation_period: Option<Period>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "ForecastException")]
    pub forecast_exception: Option<ForecastException>,
    #[serde(rename = "SupplyItem")]
    pub supply_item: Item,
}
