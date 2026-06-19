#[derive(Debug, Deserialize, Serialize)]
pub struct EndOfLifeTreatment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "TreatmentPathwayCode")]
    pub treatment_pathway_code: Option<cct::Code>,
    #[serde(default, rename = "ProcessingTypeCode")]
    pub processing_type_code: Option<cct::Code>,
    #[serde(default, rename = "ImpactCode")]
    pub impact_code: Option<cct::Code>,
    #[serde(default, rename = "TreatmentLocation")]
    pub treatment_location: Option<Location>,
}
