#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the supply (and therefore consumption) of an amount of energy or water.
///
/// UBL Dictionary Entry Name: `Energy Water Supply. Details`
///
/// Generated from XSD type `EnergyWaterSupplyType`.
pub struct EnergyWaterSupply {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An amount of energy or water consumed.
    #[serde(default, rename = "ConsumptionReport")]
    pub consumption_report: Vec<crate::ConsumptionReport>,
/// A tax on the consumption of energy or water.
    #[serde(default, rename = "EnergyTaxReport")]
    pub energy_tax_report: Vec<EnergyTaxReport>,
/// A consumption average.
    #[serde(default, rename = "ConsumptionAverage")]
    pub consumption_average: Vec<crate::ConsumptionAverage>,
/// Describes any corrections or adjustments to the supply of energy or water.
    #[serde(default, rename = "EnergyWaterConsumptionCorrection")]
    pub energy_water_consumption_correction: Vec<crate::ConsumptionCorrection>,
}
