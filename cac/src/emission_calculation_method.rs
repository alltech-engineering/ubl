#[derive(Debug, Deserialize, Serialize)]
/// A class to define how an environmental emission is calculated.
///
/// UBL Dictionary Entry Name: `Emission Calculation Method. Details`
///
/// Generated from XSD type `EmissionCalculationMethodType`.
pub struct EmissionCalculationMethod {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code signifying the method used to calculate the emission.
    #[serde(default, rename = "CalculationMethodCode")]
    pub calculation_method_code: Option<cct::Code>,
/// A code signifying whether a piece of transport equipment is full, partially full, or empty. This
/// indication is used as a parameter when calculating the environmental emission.
    #[serde(default, rename = "FullnessIndicationCode")]
    pub fullness_indication_code: Option<cct::Code>,
/// A reference to the source of the emission factor data used in the calculation of this emission.
    #[serde(default, rename = "EmissionFactorSource")]
    pub emission_factor_source: Option<cct::Text>,
/// A reference to a document that defines, publishes, or justifies the emission factor or calculation
/// method used for this emission.
    #[serde(default, rename = "EmissionFactorDocumentReference")]
    pub emission_factor_document_reference: Vec<DocumentReference>,
/// A start location from which an environmental emission is calculated.
    #[serde(default, rename = "MeasurementFromLocation")]
    pub measurement_from_location: Option<Location>,
/// An end location to which an environmental emission is calculated.
    #[serde(default, rename = "MeasurementToLocation")]
    pub measurement_to_location: Option<Location>,
/// The geographical context in which this environmental emission was calculated or for which the
/// emission factor applies, such as the country of a national database or regulatory regime.
    #[serde(default, rename = "EmissionCalculationLocation")]
    pub emission_calculation_location: Vec<Location>,
}
