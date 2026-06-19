#[derive(Debug, Deserialize, Serialize)]
/// A class to describe fuel consumption.
///
/// UBL Dictionary Entry Name: `Fuel Consumption. Details`
///
/// Generated from XSD type `FuelConsumptionType`.
pub struct FuelConsumption {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this fuel consumption.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The type of fuel, expressed as a code.
    #[serde(default, rename = "FuelTypeCode")]
    pub fuel_type_code: Option<cct::Code>,
/// The type of fuel, expressed as text.
    #[serde(default, rename = "FuelType")]
    pub fuel_type: Vec<cct::Text>,
/// The measure of this fuel consumption.
    #[serde(default, rename = "FuelConsumptionMeasure")]
    pub fuel_consumption_measure: Option<cct::Measure>,
/// One or more additional properties of the fuel being consumed.
    #[serde(default, rename = "AdditionalFuelProperty")]
    pub additional_fuel_property: Vec<FuelProperty>,
/// One or more meters of the fuel being consumed.
    #[serde(default, rename = "FuelMetering")]
    pub fuel_metering: Vec<FuelMetering>,
/// One or more environmental emissions of the fuel being measured.
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: Vec<crate::EnvironmentalEmission>,
/// The Party who provides the fuel.
    #[serde(default, rename = "FuelProviderParty")]
    pub fuel_provider_party: Option<crate::Party>,
}
