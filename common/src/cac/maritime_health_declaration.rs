#[derive(Debug, Deserialize, Serialize)]
pub struct MaritimeHealthDeclaration {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "InfectiousDiseaseCaseOnBoardIndicator")]
    pub infectious_disease_case_on_board_indicator:
        Option<super::udt::IndicatorType>,
    #[serde(default, rename = "MoreIllThanExpectedIndicator")]
    pub more_ill_than_expected_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "MedicalPractitionerConsultedIndicator")]
    pub medical_practitioner_consulted_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "StowawaysFoundOnBoardIndicator")]
    pub stowaways_found_on_board_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "SickAnimalOnBoardIndicator")]
    pub sick_animal_on_board_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "FumigatedCargoTransportIndicator")]
    pub fumigated_cargo_transport_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "SanitaryMeasuresAppliedIndicator")]
    pub sanitary_measures_applied_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ValidSanitationCertificateOnBoardIndicator")]
    pub valid_sanitation_certificate_on_board_indicator:
        Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ReinspectionRequiredIndicator")]
    pub reinspection_required_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "TotalDeadPersonQuantity")]
    pub total_dead_person_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "TotalIllPersonQuantity")]
    pub total_ill_person_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "SickAnimalDescription")]
    pub sick_animal_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "StowawayDescription")]
    pub stowaway_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "LastDrinkingWaterAnalysisDate")]
    pub last_drinking_water_analysis_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "WHOAffectedAreaVisit")]
    pub who_affected_area_visit: Vec<WhoAffectedAreaVisit>,
    #[serde(default, rename = "PersonnelHealthIncident")]
    pub personnel_health_incident: Vec<PersonnelHealthIncident>,
    #[serde(default, rename = "SanitaryMeasure")]
    pub sanitary_measure: Vec<SanitaryMeasure>,
    #[serde(default, rename = "PlaceOfReportLocation")]
    pub place_of_report_location: Option<Location>,
    #[serde(default, rename = "MedicalCertificate")]
    pub medical_certificate: Option<Certificate>,
    #[serde(default, rename = "ShipSanitationControlCertificate")]
    pub ship_sanitation_control_certificate: Option<Certificate>,
    #[serde(default, rename = "ShipSanitationControlExemptionDocumentReference")]
    pub ship_sanitation_control_exemption_document_reference:
        Vec<DocumentReference>,
}
