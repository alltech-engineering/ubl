#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionCorrection {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "CorrectionType")]
    pub correction_type: Option<cct::Text>,
    #[serde(default, rename = "CorrectionTypeCode")]
    pub correction_type_code: Option<cct::Code>,
    #[serde(default, rename = "MeterNumber")]
    pub meter_number: Option<cct::Text>,
    #[serde(default, rename = "GasPressureQuantity")]
    pub gas_pressure_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "ActualTemperatureReductionQuantity")]
    pub actual_temperature_reduction_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "NormalTemperatureReductionQuantity")]
    pub normal_temperature_reduction_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "DifferenceTemperatureReductionQuantity")]
    pub difference_temperature_reduction_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "CorrectionUnitAmount")]
    pub correction_unit_amount: Option<cct::Amount>,
    #[serde(default, rename = "ConsumptionEnergyQuantity")]
    pub consumption_energy_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "ConsumptionWaterQuantity")]
    pub consumption_water_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "CorrectionAmount")]
    pub correction_amount: Option<cct::Amount>,
}
