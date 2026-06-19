#[derive(Debug, Deserialize, Serialize)]
/// A class to describe utility consumption, including details of the environment in which consumption
/// takes place.
///
/// UBL Dictionary Entry Name: `Consumption Report. Details`
///
/// Generated from XSD type `ConsumptionReportType`.
pub struct ConsumptionReport {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this consumption report.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// The type of consumption, expressed as text.
    #[serde(default, rename = "ConsumptionType")]
    pub consumption_type: Option<cct::Text>,
/// The type of consumption, expressed as a code.
    #[serde(default, rename = "ConsumptionTypeCode")]
    pub consumption_type_code: Option<cct::Code>,
/// Text reporting utility consumption.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The total quantity consumed.
    #[serde(default, rename = "TotalConsumedQuantity")]
    pub total_consumed_quantity: Option<cct::Quantity>,
/// The basic quantity consumed, excluding additional consumption.
    #[serde(default, rename = "BasicConsumedQuantity")]
    pub basic_consumed_quantity: Option<cct::Quantity>,
/// The number of people occupying the residence covered by this report.
    #[serde(default, rename = "ResidentOccupantsNumeric")]
    pub resident_occupants_numeric: Option<cct::Numeric>,
/// The level of energy consumed, compared to the average for this residence type and the number of
/// people living in the residence, expressed as a code.
    #[serde(default, rename = "ConsumersEnergyLevelCode")]
    pub consumers_energy_level_code: Option<cct::Code>,
/// The level of energy consumed, compared to the average for this residence type and the number of
/// people living in the residence, expressed as text.
    #[serde(default, rename = "ConsumersEnergyLevel")]
    pub consumers_energy_level: Option<cct::Text>,
/// The type of residence (house, apartment, etc.) covered in this report, expressed as text.
    #[serde(default, rename = "ResidenceType")]
    pub residence_type: Option<cct::Text>,
/// The type of residence (house, apartment, etc.) covered in this report, expressed as a code.
    #[serde(default, rename = "ResidenceTypeCode")]
    pub residence_type_code: Option<cct::Code>,
/// The type of heating in the residence covered in this report, expressed as text.
    #[serde(default, rename = "HeatingType")]
    pub heating_type: Option<cct::Text>,
/// The type of heating in the residence covered in this report, expressed as a code.
    #[serde(default, rename = "HeatingTypeCode")]
    pub heating_type_code: Option<cct::Code>,
/// The period of consumption covered in this report.
    #[serde(default, rename = "Period")]
    pub period: Option<crate::Period>,
/// A reference to a document providing an explanation of this kind of report.
    #[serde(default, rename = "GuidanceDocumentReference")]
    pub guidance_document_reference: Option<crate::DocumentReference>,
/// A reference to some other document (for example, this report in another format).
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Option<crate::DocumentReference>,
/// A reference to a previous consumption report.
    #[serde(default, rename = "ConsumptionReportReference")]
    pub consumption_report_reference: Vec<ConsumptionReportReference>,
/// A report describing historical parameters relating to a specific instance of consumption.
    #[serde(default, rename = "ConsumptionHistory")]
    pub consumption_history: Vec<ConsumptionHistory>,
}
