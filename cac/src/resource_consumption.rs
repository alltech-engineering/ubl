#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceConsumption {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "ResourceTypeCode")]
    pub resource_type_code: cct::Code,
    #[serde(rename = "ConsumptionMeasure")]
    pub consumption_measure: cct::Measure,
    #[serde(default, rename = "ResourceOriginDescription")]
    pub resource_origin_description: Vec<cct::Text>,
    #[serde(default, rename = "MeasurementPeriod")]
    pub measurement_period: Option<Period>,
}
