#[derive(Debug, Deserialize, Serialize)]
/// A class to describe how an entity is expected to be treated at the end of its lifecycle, including
/// treatment pathway, processing type, location, and environmental considerations.
///
/// UBL Dictionary Entry Name: `End Of Life Treatment. Details`
///
/// Generated from XSD type `EndOfLifeTreatmentType`.
pub struct EndOfLifeTreatment {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code identifying the end-of-life pathway.
    #[serde(default, rename = "TreatmentPathwayCode")]
    pub treatment_pathway_code: Option<cct::Code>,
/// A code indicating how the product is processed at end-of-life.
    #[serde(default, rename = "ProcessingTypeCode")]
    pub processing_type_code: Option<cct::Code>,
/// A text description of the environmental impact of the selected end-of-life option.
    #[serde(default, rename = "ImpactCode")]
    pub impact_code: Option<cct::Code>,
/// A country or location where end-of-life treatment occurs.
    #[serde(default, rename = "TreatmentLocation")]
    pub treatment_location: Option<Location>,
}
