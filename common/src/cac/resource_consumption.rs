#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceConsumption {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ResourceTypeCode")]
    pub resource_type_code: super::cct::CodeType,
    #[serde(rename = "ConsumptionMeasure")]
    pub consumption_measure: super::cct::MeasureType,
    #[serde(default, rename = "ResourceOriginDescription")]
    pub resource_origin_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "MeasurementPeriod")]
    pub measurement_period: Option<Period>,
}
