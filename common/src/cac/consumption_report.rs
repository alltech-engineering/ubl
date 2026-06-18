#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionReport {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ConsumptionType")]
    pub consumption_type: Option<super::cct::TextType>,
    #[serde(default, rename = "ConsumptionTypeCode")]
    pub consumption_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "TotalConsumedQuantity")]
    pub total_consumed_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "BasicConsumedQuantity")]
    pub basic_consumed_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "ResidentOccupantsNumeric")]
    pub resident_occupants_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "ConsumersEnergyLevelCode")]
    pub consumers_energy_level_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ConsumersEnergyLevel")]
    pub consumers_energy_level: Option<super::cct::TextType>,
    #[serde(default, rename = "ResidenceType")]
    pub residence_type: Option<super::cct::TextType>,
    #[serde(default, rename = "ResidenceTypeCode")]
    pub residence_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "HeatingType")]
    pub heating_type: Option<super::cct::TextType>,
    #[serde(default, rename = "HeatingTypeCode")]
    pub heating_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
    #[serde(default, rename = "GuidanceDocumentReference")]
    pub guidance_document_reference: Option<DocumentReference>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Option<DocumentReference>,
    #[serde(default, rename = "ConsumptionReportReference")]
    pub consumption_report_reference: Vec<ConsumptionReportReference>,
    #[serde(default, rename = "ConsumptionHistory")]
    pub consumption_history: Vec<ConsumptionHistory>,
}
