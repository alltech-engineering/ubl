use serde::{Deserialize, Serialize};

include!("water_supply.rs");
include!("tax_report.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct EnergyConsumptionAllocation {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "EnergySourceTypeCode")]
    pub energy_source_type_code: cct::Code,
    #[serde(rename = "AllocatedEnergyMeasure")]
    pub allocated_energy_measure: cct::Measure,
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: Vec<crate::EnvironmentalEmission>,
}
