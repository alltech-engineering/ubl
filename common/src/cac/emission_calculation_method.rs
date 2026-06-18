#[derive(Debug, Deserialize, Serialize)]
pub struct EmissionCalculationMethod {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "CalculationMethodCode")]
    pub calculation_method_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "FullnessIndicationCode")]
    pub fullness_indication_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "EmissionFactorSource")]
    pub emission_factor_source: Option<super::cct::TextType>,
    #[serde(default, rename = "EmissionFactorDocumentReference")]
    pub emission_factor_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "MeasurementFromLocation")]
    pub measurement_from_location: Option<Location>,
    #[serde(default, rename = "MeasurementToLocation")]
    pub measurement_to_location: Option<Location>,
    #[serde(default, rename = "EmissionCalculationLocation")]
    pub emission_calculation_location: Vec<Location>,
}
