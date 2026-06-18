#[derive(Debug, Deserialize, Serialize)]
pub struct EnergyConsumptionAllocation {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "EnergySourceTypeCode")]
    pub energy_source_type_code: super::cct::CodeType,
    #[serde(rename = "AllocatedEnergyMeasure")]
    pub allocated_energy_measure: super::cct::MeasureType,
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: Vec<EnvironmentalEmission>,
}
