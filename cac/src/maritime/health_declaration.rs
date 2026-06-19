#[derive(Debug, Deserialize, Serialize)]
pub struct MaritimeHealthDeclaration {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "InfectiousDiseaseCaseOnBoardIndicator")]
    pub infectious_disease_case_on_board_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "MoreIllThanExpectedIndicator")]
    pub more_ill_than_expected_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "MedicalPractitionerConsultedIndicator")]
    pub medical_practitioner_consulted_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "StowawaysFoundOnBoardIndicator")]
    pub stowaways_found_on_board_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "SickAnimalOnBoardIndicator")]
    pub sick_animal_on_board_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "FumigatedCargoTransportIndicator")]
    pub fumigated_cargo_transport_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "SanitaryMeasuresAppliedIndicator")]
    pub sanitary_measures_applied_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "ValidSanitationCertificateOnBoardIndicator")]
    pub valid_sanitation_certificate_on_board_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "ReinspectionRequiredIndicator")]
    pub reinspection_required_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "TotalDeadPersonQuantity")]
    pub total_dead_person_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "TotalIllPersonQuantity")]
    pub total_ill_person_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "SickAnimalDescription")]
    pub sick_animal_description: Vec<cct::Text>,
    #[serde(default, rename = "StowawayDescription")]
    pub stowaway_description: Vec<cct::Text>,
    #[serde(default, rename = "LastDrinkingWaterAnalysisDate")]
    pub last_drinking_water_analysis_date: Option<udt::DateTime>,
    #[serde(default, rename = "WHOAffectedAreaVisit")]
    pub who_affected_area_visit: Vec<crate::WhoAffectedAreaVisit>,
    #[serde(default, rename = "PersonnelHealthIncident")]
    pub personnel_health_incident: Vec<crate::PersonnelHealthIncident>,
    #[serde(default, rename = "SanitaryMeasure")]
    pub sanitary_measure: Vec<crate::SanitaryMeasure>,
    #[serde(default, rename = "PlaceOfReportLocation")]
    pub place_of_report_location: Option<crate::Location>,
    #[serde(default, rename = "MedicalCertificate")]
    pub medical_certificate: Option<crate::Certificate>,
    #[serde(default, rename = "ShipSanitationControlCertificate")]
    pub ship_sanitation_control_certificate: Option<crate::Certificate>,
    #[serde(default, rename = "ShipSanitationControlExemptionDocumentReference")]
    pub ship_sanitation_control_exemption_document_reference: Vec<crate::DocumentReference>,
}
