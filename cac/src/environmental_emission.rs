#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an environmental emission.
///
/// UBL Dictionary Entry Name: `Environmental Emission. Details`
///
/// Generated from XSD type `EnvironmentalEmissionType`.
pub struct EnvironmentalEmission {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// A code specifying the type of environmental emission.
    #[serde(rename = "EnvironmentalEmissionTypeCode")]
    pub environmental_emission_type_code: cct::Code,
/// A value measurement for the environmental emission (e.g., total emissions in kg CO2)
    #[serde(rename = "ValueMeasure")]
    pub value_measure: cct::Measure,
/// A numeric factor used to calculate the value measurement (e.g., emissions per unit of activity).
    #[serde(default, rename = "ValueFactorNumeric")]
    pub value_factor_numeric: Option<cct::Numeric>,
/// The base quantity to which the value factor applies (e.g., per km, per kg, per unit produced).
    #[serde(default, rename = "ValueBaseMeasure")]
    pub value_base_measure: Option<cct::Measure>,
/// A reference to the emission reporting standard or methodological framework used to calculate and
/// report this emission.
    #[serde(default, rename = "EmissionStandardReference")]
    pub emission_standard_reference: Option<cct::Text>,
/// A code indicating the lifecycle stage to which this emission applies.
    #[serde(default, rename = "LifecycleStageCode")]
    pub lifecycle_stage_code: Option<cct::Code>,
/// The lifecycle stage to which this emission applies, expressed as a text.
    #[serde(default, rename = "LifecycleStageDescription")]
    pub lifecycle_stage_description: Vec<cct::Text>,
/// Text describing this environmental emission.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A method used to calculate the amount of this emission.
    #[serde(default, rename = "EmissionCalculationMethod")]
    pub emission_calculation_method: Vec<EmissionCalculationMethod>,
/// The period during which this environmental emission was measured or calculated.
    #[serde(default, rename = "MeasurementPeriod")]
    pub measurement_period: Option<Period>,
}
