#[derive(Debug, Deserialize, Serialize)]
pub struct CircularityProfile {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "CircularityTypeCode")]
    pub circularity_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "RecycledContentPercent")]
    pub recycled_content_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "RecyclabilityPercent")]
    pub recyclability_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "MaintenanceFrequencyCode")]
    pub maintenance_frequency_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "MaintenanceFrequencyDescription")]
    pub maintenance_frequency_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "ResourceConsumption")]
    pub resource_consumption: Vec<ResourceConsumption>,
    #[serde(default, rename = "WasteGenerated")]
    pub waste_generated: Vec<WasteGenerated>,
    #[serde(default, rename = "RepairabilityScore")]
    pub repairability_score: Vec<Score>,
    #[serde(default, rename = "EndOfLifeTreatment")]
    pub end_of_life_treatment: Option<EndOfLifeTreatment>,
    #[serde(default, rename = "ProductDocumentationDocumentReference")]
    pub product_documentation_document_reference: Vec<DocumentReference>,
}
