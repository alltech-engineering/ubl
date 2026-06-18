#[derive(Debug, Deserialize, Serialize)]
pub struct EnvironmentalEmission {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "EnvironmentalEmissionTypeCode")]
    pub environmental_emission_type_code: super::cct::CodeType,
    #[serde(rename = "ValueMeasure")]
    pub value_measure: super::cct::MeasureType,
    #[serde(default, rename = "ValueFactorNumeric")]
    pub value_factor_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "ValueBaseMeasure")]
    pub value_base_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "EmissionStandardReference")]
    pub emission_standard_reference: Option<super::cct::TextType>,
    #[serde(default, rename = "LifecycleStageCode")]
    pub lifecycle_stage_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "LifecycleStageDescription")]
    pub lifecycle_stage_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "EmissionCalculationMethod")]
    pub emission_calculation_method: Vec<EmissionCalculationMethod>,
    #[serde(default, rename = "MeasurementPeriod")]
    pub measurement_period: Option<Period>,
}
