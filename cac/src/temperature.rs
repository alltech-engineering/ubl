#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a measurement of temperature.
///
/// UBL Dictionary Entry Name: `Temperature. Details`
///
/// Generated from XSD type `TemperatureType`.
pub struct Temperature {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this temperature measurement.
    #[serde(default, rename = "AttributeID")]
    pub attribute_id: Option<cct::Identifier>,
/// The value of this temperature measurement.
    #[serde(default, rename = "Measure")]
    pub measure: Option<cct::Measure>,
/// A code describing the temperature, when not expressed as a measure.
    #[serde(default, rename = "MeasureCode")]
    pub measure_code: Option<cct::Code>,
/// Text describing this temperature measurement.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
