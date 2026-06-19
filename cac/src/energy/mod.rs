use serde::{Deserialize, Serialize};

include!("water_supply.rs");
include!("tax_report.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an allocation of energy consumption and its associated environmental emissions
///
/// UBL Dictionary Entry Name: `Energy Consumption Allocation. Details`
///
/// Generated from XSD type `EnergyConsumptionAllocationType`.
pub struct EnergyConsumptionAllocation {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code indicating the type of energy used (e.g., diesel, electricity, etc.).
    #[serde(rename = "EnergySourceTypeCode")]
    pub energy_source_type_code: cct::Code,
/// The amount of energy allocated.
    #[serde(rename = "AllocatedEnergyMeasure")]
    pub allocated_energy_measure: cct::Measure,
/// The corresponding Environmental Emissions associated with this allocation.
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: Vec<crate::EnvironmentalEmission>,
}
