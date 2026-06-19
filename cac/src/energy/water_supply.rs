#[derive(Debug, Deserialize, Serialize)]
pub struct EnergyWaterSupply {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ConsumptionReport")]
    pub consumption_report: Vec<crate::ConsumptionReport>,
    #[serde(default, rename = "EnergyTaxReport")]
    pub energy_tax_report: Vec<EnergyTaxReport>,
    #[serde(default, rename = "ConsumptionAverage")]
    pub consumption_average: Vec<crate::ConsumptionAverage>,
    #[serde(default, rename = "EnergyWaterConsumptionCorrection")]
    pub energy_water_consumption_correction: Vec<crate::ConsumptionCorrection>,
}
