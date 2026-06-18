#[derive(Debug, Deserialize, Serialize)]
pub struct FuelConsumption {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "FuelTypeCode")]
    pub fuel_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "FuelType")]
    pub fuel_type: Vec<super::cct::TextType>,
    #[serde(default, rename = "FuelConsumptionMeasure")]
    pub fuel_consumption_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "AdditionalFuelProperty")]
    pub additional_fuel_property: Vec<FuelProperty>,
    #[serde(default, rename = "FuelMetering")]
    pub fuel_metering: Vec<FuelMetering>,
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: Vec<EnvironmentalEmission>,
    #[serde(default, rename = "FuelProviderParty")]
    pub fuel_provider_party: Option<Party>,
}
