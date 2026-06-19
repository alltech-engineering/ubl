#[derive(Debug, Deserialize, Serialize)]
pub struct FuelConsumption {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "FuelTypeCode")]
    pub fuel_type_code: Option<cct::Code>,
    #[serde(default, rename = "FuelType")]
    pub fuel_type: Vec<cct::Text>,
    #[serde(default, rename = "FuelConsumptionMeasure")]
    pub fuel_consumption_measure: Option<cct::Measure>,
    #[serde(default, rename = "AdditionalFuelProperty")]
    pub additional_fuel_property: Vec<FuelProperty>,
    #[serde(default, rename = "FuelMetering")]
    pub fuel_metering: Vec<FuelMetering>,
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: Vec<crate::EnvironmentalEmission>,
    #[serde(default, rename = "FuelProviderParty")]
    pub fuel_provider_party: Option<crate::Party>,
}
