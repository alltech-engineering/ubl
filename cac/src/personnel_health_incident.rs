#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a health incident involving crew or other personnel.
///
/// UBL Dictionary Entry Name: `Personnel Health Incident. Details`
///
/// Generated from XSD type `PersonnelHealthIncidentType`.
pub struct PersonnelHealthIncident {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this personal health incident.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// The date when the person joined the ship.
    #[serde(default, rename = "JoinedShipDate")]
    pub joined_ship_date: Option<udt::DateTime>,
/// A text decribing the nature of the illness.
    #[serde(default, rename = "NatureOfIllnessDescription")]
    pub nature_of_illness_description: Vec<cct::Text>,
/// The first date of the health incident.
    #[serde(default, rename = "OnsetDate")]
    pub onset_date: Option<udt::DateTime>,
/// An indicator of whether this personal health incident has been reported to a medical officer (true)
/// or not (false).
    #[serde(default, rename = "ReportedToMedicalOfficerIndicator")]
    pub reported_to_medical_officer_indicator: Option<udt::Indicator>,
/// A text describing the given treatment.
    #[serde(default, rename = "GivenTreatmentDescription")]
    pub given_treatment_description: Vec<cct::Text>,
/// Indicates whether the person is still ill (true) or not (false).
    #[serde(default, rename = "StillIllIndicator")]
    pub still_ill_indicator: Option<udt::Indicator>,
/// Indicates whether the person died from this health incident (true) or not (false).
    #[serde(default, rename = "DiedIndicator")]
    pub died_indicator: Option<udt::Indicator>,
/// Indicates whether the person is still on board (true) or not (false).
    #[serde(default, rename = "StillOnBoardIndicator")]
    pub still_on_board_indicator: Option<udt::Indicator>,
/// Indicates whether the person has been evacuated (true) or not (false).
    #[serde(default, rename = "EvacuatedIndicator")]
    pub evacuated_indicator: Option<udt::Indicator>,
/// Indicates whether the person has been buired at sea (true) or not (false).
    #[serde(default, rename = "BuriedAtSeaIndicator")]
    pub buried_at_sea_indicator: Option<udt::Indicator>,
/// Any additional information that is not included elsewhere, expressed as text.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The person associated to this health incident.
    #[serde(default, rename = "Person")]
    pub person: Option<Person>,
}
