#[derive(Debug, Deserialize, Serialize)]
pub struct EnvironmentalEmission {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "EnvironmentalEmissionTypeCode")]
    pub environmental_emission_type_code: cct::Code,
    #[serde(rename = "ValueMeasure")]
    pub value_measure: cct::Measure,
    #[serde(default, rename = "ValueFactorNumeric")]
    pub value_factor_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "ValueBaseMeasure")]
    pub value_base_measure: Option<cct::Measure>,
    #[serde(default, rename = "EmissionStandardReference")]
    pub emission_standard_reference: Option<cct::Text>,
    #[serde(default, rename = "LifecycleStageCode")]
    pub lifecycle_stage_code: Option<cct::Code>,
    #[serde(default, rename = "LifecycleStageDescription")]
    pub lifecycle_stage_description: Vec<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "EmissionCalculationMethod")]
    pub emission_calculation_method: Vec<EmissionCalculationMethod>,
    #[serde(default, rename = "MeasurementPeriod")]
    pub measurement_period: Option<Period>,
}
