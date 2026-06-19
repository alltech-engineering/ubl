#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionReport {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "ConsumptionType")]
    pub consumption_type: Option<cct::Text>,
    #[serde(default, rename = "ConsumptionTypeCode")]
    pub consumption_type_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "TotalConsumedQuantity")]
    pub total_consumed_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "BasicConsumedQuantity")]
    pub basic_consumed_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "ResidentOccupantsNumeric")]
    pub resident_occupants_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "ConsumersEnergyLevelCode")]
    pub consumers_energy_level_code: Option<cct::Code>,
    #[serde(default, rename = "ConsumersEnergyLevel")]
    pub consumers_energy_level: Option<cct::Text>,
    #[serde(default, rename = "ResidenceType")]
    pub residence_type: Option<cct::Text>,
    #[serde(default, rename = "ResidenceTypeCode")]
    pub residence_type_code: Option<cct::Code>,
    #[serde(default, rename = "HeatingType")]
    pub heating_type: Option<cct::Text>,
    #[serde(default, rename = "HeatingTypeCode")]
    pub heating_type_code: Option<cct::Code>,
    #[serde(default, rename = "Period")]
    pub period: Option<crate::Period>,
    #[serde(default, rename = "GuidanceDocumentReference")]
    pub guidance_document_reference: Option<crate::DocumentReference>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Option<crate::DocumentReference>,
    #[serde(default, rename = "ConsumptionReportReference")]
    pub consumption_report_reference: Vec<ConsumptionReportReference>,
    #[serde(default, rename = "ConsumptionHistory")]
    pub consumption_history: Vec<ConsumptionHistory>,
}
