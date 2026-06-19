#[derive(Debug, Deserialize, Serialize)]
/// A class to desccribe a maritime health declaration.
///
/// UBL Dictionary Entry Name: `Maritime Health Declaration. Details`
///
/// Generated from XSD type `MaritimeHealthDeclarationType`.
pub struct MaritimeHealthDeclaration {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this maritime health declaration.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// Indicates whether there is an infectious diasase case on board (true) or not (false).
    #[serde(default, rename = "InfectiousDiseaseCaseOnBoardIndicator")]
    pub infectious_disease_case_on_board_indicator: Option<udt::Indicator>,
/// Indicates whether a sick person is more ill than extected (true) or not (false).
    #[serde(default, rename = "MoreIllThanExpectedIndicator")]
    pub more_ill_than_expected_indicator: Option<udt::Indicator>,
/// Indicates whether a medical practioner has been consulted (true) or not (false).
    #[serde(default, rename = "MedicalPractitionerConsultedIndicator")]
    pub medical_practitioner_consulted_indicator: Option<udt::Indicator>,
/// Indicates whether stowaways have been found on board (true) or not (false).
    #[serde(default, rename = "StowawaysFoundOnBoardIndicator")]
    pub stowaways_found_on_board_indicator: Option<udt::Indicator>,
/// Indicates whether a sick animal is on board (true) or not (false).
    #[serde(default, rename = "SickAnimalOnBoardIndicator")]
    pub sick_animal_on_board_indicator: Option<udt::Indicator>,
/// Indicates whether the cargo under transport has been fumigated (true) or not (false).
    #[serde(default, rename = "FumigatedCargoTransportIndicator")]
    pub fumigated_cargo_transport_indicator: Option<udt::Indicator>,
/// Indicates whether sanity measures are applied (true) or not (false).
    #[serde(default, rename = "SanitaryMeasuresAppliedIndicator")]
    pub sanitary_measures_applied_indicator: Option<udt::Indicator>,
/// Indicates whether a valid sanitary certificate is on board (true) or not (false).
    #[serde(default, rename = "ValidSanitationCertificateOnBoardIndicator")]
    pub valid_sanitation_certificate_on_board_indicator: Option<udt::Indicator>,
/// Indicates whether a reinspaction is required (true) or not (false).
    #[serde(default, rename = "ReinspectionRequiredIndicator")]
    pub reinspection_required_indicator: Option<udt::Indicator>,
/// Specifies the total number of dead persons on board the vessel.
    #[serde(default, rename = "TotalDeadPersonQuantity")]
    pub total_dead_person_quantity: Option<cct::Quantity>,
/// Specifies the total number of ill persons on board the vessel.
    #[serde(default, rename = "TotalIllPersonQuantity")]
    pub total_ill_person_quantity: Option<cct::Quantity>,
/// Describes any sick animals on board the vessel.
    #[serde(default, rename = "SickAnimalDescription")]
    pub sick_animal_description: Vec<cct::Text>,
/// Describes any stowaways on board the vessel.
    #[serde(default, rename = "StowawayDescription")]
    pub stowaway_description: Vec<cct::Text>,
/// The date when the last drinking water analysis was made.
    #[serde(default, rename = "LastDrinkingWaterAnalysisDate")]
    pub last_drinking_water_analysis_date: Option<udt::DateTime>,
/// A WHO Affected Area visit related to this maritime health declaration.
    #[serde(default, rename = "WHOAffectedAreaVisit")]
    pub who_affected_area_visit: Vec<crate::WhoAffectedAreaVisit>,
/// A personal health incident related to this maritime health declaration.
    #[serde(default, rename = "PersonnelHealthIncident")]
    pub personnel_health_incident: Vec<crate::PersonnelHealthIncident>,
/// A sanitary meassure for this health declaration.
    #[serde(default, rename = "SanitaryMeasure")]
    pub sanitary_measure: Vec<crate::SanitaryMeasure>,
/// The location where this maritime health declaration is reported.
    #[serde(default, rename = "PlaceOfReportLocation")]
    pub place_of_report_location: Option<crate::Location>,
/// The medical certificate for this maritime health declaration.
    #[serde(default, rename = "MedicalCertificate")]
    pub medical_certificate: Option<crate::Certificate>,
/// A certificate describing the sanitation control of this maritime health certificate.
    #[serde(default, rename = "ShipSanitationControlCertificate")]
    pub ship_sanitation_control_certificate: Option<crate::Certificate>,
/// A reference to a document evidencing the exemption of a ship sanitation control certificate, when
/// absent.
    #[serde(default, rename = "ShipSanitationControlExemptionDocumentReference")]
    pub ship_sanitation_control_exemption_document_reference: Vec<crate::DocumentReference>,
}
