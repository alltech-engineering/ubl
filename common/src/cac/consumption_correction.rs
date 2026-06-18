#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionCorrection {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "CorrectionType")]
    pub correction_type: Option<super::cct::TextType>,
    #[serde(default, rename = "CorrectionTypeCode")]
    pub correction_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "MeterNumber")]
    pub meter_number: Option<super::cct::TextType>,
    #[serde(default, rename = "GasPressureQuantity")]
    pub gas_pressure_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "ActualTemperatureReductionQuantity")]
    pub actual_temperature_reduction_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "NormalTemperatureReductionQuantity")]
    pub normal_temperature_reduction_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "DifferenceTemperatureReductionQuantity")]
    pub difference_temperature_reduction_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "CorrectionUnitAmount")]
    pub correction_unit_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "ConsumptionEnergyQuantity")]
    pub consumption_energy_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "ConsumptionWaterQuantity")]
    pub consumption_water_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "CorrectionAmount")]
    pub correction_amount: Option<super::cct::AmountType>,
}
