#[derive(Debug, Deserialize, Serialize)]
/// The Statement of correction, for examples heating correction.
///
/// UBL Dictionary Entry Name: `Consumption Correction. Details`
///
/// Generated from XSD type `ConsumptionCorrectionType`.
pub struct ConsumptionCorrection {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// Statement for the correction type.
    #[serde(default, rename = "CorrectionType")]
    pub correction_type: Option<cct::Text>,
/// Statement at the code for the correction type.
    #[serde(default, rename = "CorrectionTypeCode")]
    pub correction_type_code: Option<cct::Code>,
/// Statement for meter number.
    #[serde(default, rename = "MeterNumber")]
    pub meter_number: Option<cct::Text>,
/// Correction of the gas pressure.
    #[serde(default, rename = "GasPressureQuantity")]
    pub gas_pressure_quantity: Option<cct::Quantity>,
/// Statement for the actuel heating correction temperature.
    #[serde(default, rename = "ActualTemperatureReductionQuantity")]
    pub actual_temperature_reduction_quantity: Option<cct::Quantity>,
/// Statement for the standard for heating correction temperature.
    #[serde(default, rename = "NormalTemperatureReductionQuantity")]
    pub normal_temperature_reduction_quantity: Option<cct::Quantity>,
/// Deviation from standard heating correction.
    #[serde(default, rename = "DifferenceTemperatureReductionQuantity")]
    pub difference_temperature_reduction_quantity: Option<cct::Quantity>,
/// Description related to the corrections.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Correction per MWH per degree C.
    #[serde(default, rename = "CorrectionUnitAmount")]
    pub correction_unit_amount: Option<cct::Amount>,
/// Your consumpt for district heating energy.
    #[serde(default, rename = "ConsumptionEnergyQuantity")]
    pub consumption_energy_quantity: Option<cct::Quantity>,
/// Your consumpt for district heating water.
    #[serde(default, rename = "ConsumptionWaterQuantity")]
    pub consumption_water_quantity: Option<cct::Quantity>,
/// Your correction for heating correction.
    #[serde(default, rename = "CorrectionAmount")]
    pub correction_amount: Option<cct::Amount>,
}
